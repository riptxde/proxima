--/ Definitions /--
-- Constants
local PROXIMA_URL = 'ws://localhost:13376'
local RECONNECT_DELAY = 5
local LOG_INFO = 0
local LOG_SUCCESS = 1
local LOG_WARNING = 2
local LOG_ERROR = 3

-- Services
local Players = game:GetService('Players')
local HttpService = game:GetService('HttpService')

-- Objects
local LocalPlayer = Players.LocalPlayer
local Socket = nil

-- User Data (will be set during registration)
local Username = nil

-- State
local Reconnecting = false

--/ Functions /--
local function Log(Level, ...)
    if not Socket then
        return
    end

    local Args = {...}
    for i = 1, #Args do
        Args[i] = tostring(Args[i])
    end
    local Message = table.concat(Args, ' ')

    pcall(function()
        local Payload = HttpService:JSONEncode({
            type = 'log',
            level = Level,
            message = Message
        })
        Socket:Send(Payload)
    end)
end

local function Ready()
    if not Socket then
        return
    end

    pcall(function()
        local Message = HttpService:JSONEncode({
            type = 'ready'
        })
        Socket:Send(Message)
    end)
end

local function Register()
    if not Socket then
        return
    end

    -- Wait for LocalPlayer if it doesn't exist yet
    while not LocalPlayer do
        Players.PlayerAdded:Wait()
        LocalPlayer = Players.LocalPlayer
    end

    -- Set username from LocalPlayer or generate GUID
    Username = LocalPlayer.Name or HttpService:GenerateGUID(false)

    pcall(function()
        local Message = HttpService:JSONEncode({
            type = 'register',
            username = Username
        })
        Socket:Send(Message)
    end)
end

local function Execute(Script)
    local Func, Err = loadstring(Script)

    if not Func then
        Log(LOG_ERROR, tostring(Err))
        return
    end

    xpcall(Func, function(Err)
        Log(LOG_ERROR, tostring(Err))
    end)
end

local function HandleMessage(Message)
    local Success, Data = pcall(function()
        return HttpService:JSONDecode(Message)
    end)

    if not Success then
        return
    end

    if Data.type == 'ping' then
        pcall(function()
            local Message = HttpService:JSONEncode({
                type = 'pong'
            })
            Socket:Send(Message)
        end)
    elseif Data.type == 'execute' then
        Execute(Data.script)
    end
end

local function Connect()
    if Reconnecting then
        return
    end

    local Success, Result = pcall(function()
        return WebSocket.connect(PROXIMA_URL)
    end)

    if not Success then
        Reconnecting = true
        wait(RECONNECT_DELAY)
        Reconnecting = false
        Connect()
        return
    end

    Socket = Result

    Socket.OnMessage:Connect(function(Message)
        HandleMessage(Message)
    end)

    Socket.OnClose:Connect(function()
        Socket = nil
        Reconnecting = true
        wait(RECONNECT_DELAY)
        Reconnecting = false
        Connect()
    end)

    -- Send ready message immediately after handlers are set up
    Ready()

    -- Register in a separate thread to avoid blocking auto-execute scripts
    coroutine.wrap(Register)()
end

--/ Main /--
-- Setup console functions in getgenv()
local Env = getgenv()

Env.printconsole = function(...)
    Log(LOG_INFO, ...)
end

Env.successconsole = function(...)
    Log(LOG_SUCCESS, ...)
end

Env.warnconsole = function(...)
    Log(LOG_WARNING, ...)
end

Env.errorconsole = function(...)
    Log(LOG_ERROR, ...)
end

Connect()
