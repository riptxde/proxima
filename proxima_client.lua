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

local function Pong()
    pcall(function()
        local Message = HttpService:JSONEncode({
            type = 'pong'
        })
        Socket:Send(Message)
    end)
end

local function Exec(Script)
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
        Pong()
    elseif Data.type == 'exec' then
        Exec(Data.script)
    elseif Data.type == 'exp_start' then
        ExpStart()
    elseif Data.type == 'exp_stop' then
        ExpStop()
    elseif Data.type == 'exp_get_tree' then
        ExpGetTree(Data.expandedIds or {})
    elseif Data.type == 'exp_get_properties' then
        ExpGetProperties(Data.id, Data.properties or {}, Data.specialProperties or {})
    elseif Data.type == 'exp_search' then
        ExpSearch(Data.query, Data.searchBy or 'both', Data.limit or 1000)
    elseif Data.type == 'exp_decompile' then
        ExpDecompile(Data.id)
    elseif Data.type == 'rspy_start' then
        RspyStart()
    elseif Data.type == 'rspy_stop' then
        RspyStop()
    elseif Data.type == 'rspy_decompile' then
        RspyDecompile(Data.scriptPath)
    elseif Data.type == 'rspy_generate_code' then
        RspyGenerateCode(Data.callId, Data.name, Data.path, Data.remoteType, Data.direction, Data.arguments or {})
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
        if ExplorerActive then
            HandleStopExplorer()
        end

        if RemoteSpyActive then
            RspyStop()
        end

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

--/ Explorer /--
local ExplorerActive = false
local Instances = {}
local IdToInstance = {}
local NextId = 1
local CurrentExpandedIds = {}
local LastVisibleTreeHash = nil
local ChangeDebounceTimer = nil
local DescendantAddedConnection = nil
local DescendantRemovingConnection = nil

-- Explorer Functions
local function SendExplorerMessage(Data)
    if not Socket then
        return
    end

    pcall(function()
        local Json = HttpService:JSONEncode(Data)
        Socket:Send(Json)
    end)
end

local function EscapeStringLiteral(Name)
    -- Determine which quote style to use based on what's in the string
    local HasSingleQuote = string.find(Name, "'", 1, true) ~= nil
    local HasDoubleQuote = string.find(Name, '"', 1, true) ~= nil

    if HasSingleQuote and HasDoubleQuote then
        -- Has both quotes - use double quotes and escape them with backslash
        local Escaped = string.gsub(Name, '\\', '\\\\')  -- Escape backslashes first
        Escaped = string.gsub(Escaped, '"', '\\"')       -- Escape double quotes
        return '"' .. Escaped .. '"'
    elseif HasDoubleQuote then
        -- Has double quotes only - use single quotes
        return "'" .. Name .. "'"
    else
        -- Has no double quotes or only single quotes - use double quotes
        return '"' .. Name .. '"'
    end
end

local function EscapeInstanceName(Name)
    -- Check if name is a valid Lua identifier
    -- Must start with letter or underscore, followed only by letters, numbers, or underscores
    -- Pattern: ^[A-Za-z_][A-Za-z0-9_]*$
    local IsValidIdentifier = true

    -- Check first character (must be letter or underscore)
    local FirstChar = string.sub(Name, 1, 1)
    if not string.match(FirstChar, "[A-Za-z_]") then
        IsValidIdentifier = false
    end

    -- Check remaining characters (must be letter, number, or underscore)
    if IsValidIdentifier and #Name > 1 then
        for i = 2, #Name do
            local Char = string.sub(Name, i, i)
            if not string.match(Char, "[A-Za-z0-9_]") then
                IsValidIdentifier = false
                break
            end
        end
    end

    if IsValidIdentifier then
        -- Can use dot notation (e.g., Workspace.Part)
        return "." .. Name
    else
        -- Need bracket notation with escaped string
        return "[" .. EscapeStringLiteral(Name) .. "]"
    end
end

local function GetOrCreateId(Instance)
    if Instances[Instance] then
        return Instances[Instance]
    end

    local Id = NextId
    NextId = NextId + 1
    Instances[Instance] = Id
    IdToInstance[Id] = Instance
    return Id
end

local function GetId(Instance)
    return Instances[Instance]
end

local function GetInstance(Id)
    return IdToInstance[Id]
end

local function BuildVisibleTree(ExpandedIds)
    local Tree = {}

    local function AddNode(Instance)
        local Id = GetOrCreateId(Instance)
        local Children = Instance:GetChildren()
        local IsExpanded = ExpandedIds[Id]

        local Node = {
            id = Id,
            n = Instance.Name,
            c = Instance.ClassName,
            h = #Children > 0,
            children = {}
        }

        if IsExpanded and #Children > 0 then
            for I = 1, #Children do
                local Child = Children[I]
                table.insert(Node.children, AddNode(Child))
            end
        end

        return Node
    end

    local RootChildren = game:GetChildren()
    for I = 1, #RootChildren do
        table.insert(Tree, AddNode(RootChildren[I]))
    end

    return Tree
end

local function HashTree(Tree)
    local Json = HttpService:JSONEncode(Tree)
    return string.len(Json) .. string.sub(Json, 1, 10) .. string.sub(Json, -10)
end

local function CheckForVisibleChanges()
    if #CurrentExpandedIds == 0 then
        return
    end

    local ExpandedMap = {}
    for I = 1, #CurrentExpandedIds do
        ExpandedMap[CurrentExpandedIds[I]] = true
    end

    local CurrentTree = BuildVisibleTree(ExpandedMap)
    local CurrentHash = HashTree(CurrentTree)

    if CurrentHash ~= LastVisibleTreeHash then
        LastVisibleTreeHash = CurrentHash
        SendExplorerMessage({
            type = 'exp_tree_changed'
        })
    end

    ChangeDebounceTimer = nil
end

local function ScheduleChangeCheck()
    if ChangeDebounceTimer then
        task.cancel(ChangeDebounceTimer)
    end

    ChangeDebounceTimer = task.delay(0.3, CheckForVisibleChanges)
end

local function OnDescendantChanged()
    if ExplorerActive then
        ScheduleChangeCheck()
    end
end

function ExpStart()
    if ExplorerActive then
        return
    end

    ExplorerActive = true
    Instances = {}
    IdToInstance = {}
    NextId = 1
    CurrentExpandedIds = {}
    LastVisibleTreeHash = nil

    -- Setup game as ID 0
    Instances[game] = 0
    IdToInstance[0] = game

    -- Setup change listeners (store connections for cleanup)
    DescendantAddedConnection = game.DescendantAdded:Connect(OnDescendantChanged)
    DescendantRemovingConnection = game.DescendantRemoving:Connect(OnDescendantChanged)

    Log(LOG_SUCCESS, 'Explorer started successfully')
end

function ExpStop()
    if not ExplorerActive then
        return
    end

    ExplorerActive = false

    -- Cancel pending change checks
    if ChangeDebounceTimer then
        task.cancel(ChangeDebounceTimer)
        ChangeDebounceTimer = nil
    end

    -- Disconnect event listeners
    if DescendantAddedConnection then
        DescendantAddedConnection:Disconnect()
        DescendantAddedConnection = nil
    end

    if DescendantRemovingConnection then
        DescendantRemovingConnection:Disconnect()
        DescendantRemovingConnection = nil
    end

    -- Clear state
    Instances = {}
    IdToInstance = {}
    CurrentExpandedIds = {}
    LastVisibleTreeHash = nil

    Log(LOG_INFO, 'Explorer disconnected successfully')
end

function ExpGetTree(ExpandedIds)
    if not ExplorerActive then
        return
    end

    CurrentExpandedIds = ExpandedIds

    local ExpandedMap = {}
    for I = 1, #ExpandedIds do
        ExpandedMap[ExpandedIds[I]] = true
    end

    local Tree = BuildVisibleTree(ExpandedMap)
    LastVisibleTreeHash = HashTree(Tree)

    SendExplorerMessage({
        type = 'exp_tree',
        nodes = Tree
    })
end

function ExpGetProperties(Id, Properties, SpecialProperties)
    if not ExplorerActive then
        return
    end

    local Instance = GetInstance(Id)

    if not Instance then
        SendExplorerMessage({
            type = 'error',
            request = 'get_explorer_properties',
            message = 'Invalid ID'
        })
        return
    end

    local Props = {}
    local SpecialProps = {}

    -- Check if gethiddenproperty is available for special properties
    local HasGetHiddenProperty = type(gethiddenproperty) == 'function'

    -- Get regular properties
    for _, PropMetadata in ipairs(Properties) do
        local PropName = PropMetadata.name
        local Success, Value = pcall(function()
            return Instance[PropName]
        end)

        if Success then
            Props[PropName] = {
                value = tostring(Value),
                type = typeof(Value),
                class = Instance.ClassName,
                deprecated = PropMetadata.deprecated,
                hidden = PropMetadata.hidden,
                notScriptable = PropMetadata.not_scriptable,
                example = {
                    get = "local value = instance." .. PropName,
                    set = "instance." .. PropName .. " = value"
                }
            }
        end
    end

    -- Get special properties (hidden/not scriptable) if executor supports it
    if HasGetHiddenProperty then
        for _, PropMetadata in ipairs(SpecialProperties) do
            local PropName = PropMetadata.name
            local Success, Value = pcall(function()
                return gethiddenproperty(Instance, PropName)
            end)

            if Success then
                SpecialProps[PropName] = {
                    value = tostring(Value),
                    type = typeof(Value),
                    class = Instance.ClassName,
                    deprecated = PropMetadata.deprecated,
                    hidden = PropMetadata.hidden,
                    notScriptable = PropMetadata.not_scriptable,
                    example = {
                        get = "local value = gethiddenproperty(instance, '" .. PropName .. "')",
                        set = "sethiddenproperty(instance, '" .. PropName .. "', value)"
                    }
                }
            end
        end
    end

    SendExplorerMessage({
        type = 'exp_properties',
        id = Id,
        props = Props,
        specialProps = SpecialProps
    })
end

function ExpSearch(Query, SearchBy, Limit)
    if not ExplorerActive then
        return
    end

    local Results = {}
    local Descendants = game:GetDescendants()
    local Count = 0
    local Limited = false
    local QueryLower = string.lower(Query)

    -- Search all descendants until limit is reached
    for I = 1, #Descendants do
        local Descendant = Descendants[I]
        local Matches = false

        -- Check if instance matches search criteria
        if SearchBy == 'name' or SearchBy == 'both' then
            if string.find(string.lower(Descendant.Name), QueryLower, 1, true) then
                Matches = true
            end
        end

        if (SearchBy == 'classname' or SearchBy == 'both') and not Matches then
            if string.find(string.lower(Descendant.ClassName), QueryLower, 1, true) then
                Matches = true
            end
        end

        if Matches then
            Count = Count + 1

            if Count <= Limit then
                -- Create ID for this match
                local MatchId = GetOrCreateId(Descendant)

                -- Build full absolute path by creating IDs for all ancestors up to game
                local PathParts = {}
                local PathStringParts = {}
                local Current = Descendant.Parent

                while Current and Current ~= game do
                    -- Always create ID for ancestor to ensure full path
                    local AncestorId = GetOrCreateId(Current)
                    table.insert(PathParts, 1, AncestorId)
                    Current = Current.Parent
                end

                -- Add the instance itself to the path
                table.insert(PathParts, MatchId)

                -- Build proper Lua indexing path from game root
                local PathRoot = "game"
                local RootInstance = nil

                -- Special case: if the descendant itself is a direct child of game
                if Descendant.Parent == game then
                    RootInstance = Descendant
                else
                    -- Build path for ancestors
                    Current = Descendant.Parent
                    while Current and Current ~= game do
                        table.insert(PathStringParts, 1, EscapeInstanceName(Current.Name))
                        RootInstance = Current
                        Current = Current.Parent
                    end
                end

                -- Check if the root is a direct child of game (service)
                if RootInstance and RootInstance.Parent == game then
                    -- Check if it's Workspace - use workspace global
                    if RootInstance:IsA("Workspace") then
                        PathRoot = "workspace"
                        -- Only remove from parts if we added it (i.e., not the descendant itself)
                        if RootInstance ~= Descendant then
                            table.remove(PathStringParts, 1)  -- Remove the Workspace part since it's now the root
                        end
                    else
                        -- Check if it's a service by verifying GetService returns the same instance
                        local Success, Service = pcall(function()
                            return game:GetService(RootInstance.ClassName)
                        end)

                        if Success and Service == RootInstance then
                            -- Use GetService with proper string escaping
                            PathRoot = "game:GetService(" .. EscapeStringLiteral(RootInstance.ClassName) .. ")"
                            -- Only remove from parts if we added it (i.e., not the descendant itself)
                            if RootInstance ~= Descendant then
                                table.remove(PathStringParts, 1)  -- Remove the service name since GetService handles it
                            end
                        end
                    end
                end

                -- Add the instance itself to the path string (if not already handled as root)
                if RootInstance ~= Descendant then
                    table.insert(PathStringParts, EscapeInstanceName(Descendant.Name))
                end

                -- Build path string (e.g., 'workspace.Model["Part with spaces"]')
                local PathString = PathRoot
                if #PathStringParts > 0 then
                    PathString = PathRoot .. table.concat(PathStringParts, "")
                end

                local Children = Descendant:GetChildren()

                table.insert(Results, {
                    id = MatchId,
                    n = Descendant.Name,
                    c = Descendant.ClassName,
                    p = PathParts,
                    s = PathString,
                    h = #Children > 0
                })
            else
                Limited = true
            end
        end
    end

    SendExplorerMessage({
        type = 'exp_search_results',
        query = Query,
        results = Results,
        total = Count,
        limited = Limited
    })
end

function ExpDecompile(Id)
    if not ExplorerActive then
        return
    end

    -- Get the instance by ID
    local Instance = IdToInstance[Id]
    if not Instance then
        SendExplorerMessage({
            type = 'exp_decompiled',
            id = Id,
            source = '-- Instance not found'
        })
        return
    end

    -- Check if instance is a script type
    if not (Instance:IsA('LocalScript') or Instance:IsA('ModuleScript')) then
        SendExplorerMessage({
            type = 'exp_decompiled',
            id = Id,
            source = '-- Not a script instance'
        })
        return
    end

    -- Try to decompile using the decompile function if available
    local DecompiledSource
    if decompile then
        local Success, Result = pcall(function()
            return decompile(Instance)
        end)

        if Success then
            DecompiledSource = Result
        else
            DecompiledSource = '-- Decompile failed: ' .. tostring(Result)
        end
    else
        DecompiledSource = '-- Your executor does not support script decompilation'
    end

    SendExplorerMessage({
        type = 'exp_decompiled',
        id = Id,
        source = DecompiledSource
    })
end

--/ Remote Spy /--
local RemoteSpyActive = false
local RemoteCallCounter = 0
local RemoteInstances = {} -- Maps remote instance -> ID
local NextRemoteId = 1

-- Remote Spy Functions
local function SendRemoteSpyMessage(Data)
    if not Socket then
        return
    end

    pcall(function()
        local Json = HttpService:JSONEncode(Data)
        Socket:Send(Json)
    end)
end

function RspyStart()
    RemoteSpyActive = true
    Log(LOG_SUCCESS, 'Remote spy started')

    -- Start sending dummy remote calls
    task.spawn(function()
        while RemoteSpyActive do
            task.wait(math.random(2, 5)) -- Random interval between 2-5 seconds

            if not RemoteSpyActive then
                break
            end

            -- Generate a random dummy remote call
            SendDummyRemoteCall()
        end
    end)
end

function RspyStop()
    RemoteSpyActive = false
    Log(LOG_SUCCESS, 'Remote spy stopped')
end

function SendDummyRemoteCall()
    RemoteCallCounter = RemoteCallCounter + 1

    -- Dummy remotes to cycle through (each represents a unique "instance")
    local DummyRemotes = {
        {
            id = 1,
            name = 'PlayerDataRequest',
            path = 'ReplicatedStorage.Remotes.PlayerData',
            type = 'RemoteFunction'
        },
        {
            id = 2,
            name = 'UpdateInventory',
            path = 'ReplicatedStorage.Remotes.Inventory.Update',
            type = 'RemoteEvent'
        },
        {
            id = 3,
            name = 'FireWeapon',
            path = 'ReplicatedStorage.Combat.WeaponFire',
            type = 'RemoteEvent'
        },
        {
            id = 4,
            name = 'TeleportPlayer',
            path = 'ReplicatedStorage.Remotes.Teleport',
            type = 'RemoteFunction'
        },
    }

    -- Dummy calling scripts
    local DummyScripts = {
        {
            name = 'LocalScript',
            path = 'StarterPlayer.StarterPlayerScripts.LocalScript'
        },
        {
            name = 'PlayerController',
            path = 'StarterPlayer.StarterPlayerScripts.Controllers.PlayerController'
        },
        {
            name = 'WeaponHandler',
            path = 'StarterPlayer.StarterPlayerScripts.WeaponHandler'
        },
    }

    -- Pick random remote and script
    local Remote = DummyRemotes[math.random(1, #DummyRemotes)]
    local Script = DummyScripts[math.random(1, #DummyScripts)]
    local Direction = math.random() > 0.5 and 'outgoing' or 'incoming'

    -- Generate dummy arguments
    local DummyArgs = {}
    local ArgCount = math.random(1, 3)
    for I = 1, ArgCount do
        local ArgTypes = {'string', 'number', 'boolean', 'table'}
        local ArgType = ArgTypes[math.random(1, #ArgTypes)]

        local Value
        if ArgType == 'string' then
            Value = '"Example ' .. I .. '"'
        elseif ArgType == 'number' then
            Value = tostring(math.random(1, 100))
        elseif ArgType == 'boolean' then
            Value = math.random() > 0.5 and 'true' or 'false'
        else
            Value = '{ ... }'
        end

        table.insert(DummyArgs, {
            type = ArgType,
            value = Value
        })
    end

    -- Build remote call event
    local CallEvent = {
        type = 'rspy_call',
        remoteId = Remote.id,
        name = Remote.name,
        path = Remote.path,
        remoteType = Remote.type,
        direction = Direction,
        timestamp = os.date('!%Y-%m-%dT%H:%M:%S') .. '.000Z',
        arguments = DummyArgs,
        callingScript = Script.name,
        callingScriptPath = Script.path
    }

    -- Add return value for RemoteFunctions with incoming direction
    if Remote.type == 'RemoteFunction' and Direction == 'incoming' then
        CallEvent.returnValue = {
            type = math.random() > 0.5 and 'boolean' or 'table',
            value = math.random() > 0.5 and 'true' or '{ success = true }'
        }
    end

    SendRemoteSpyMessage(CallEvent)
end

function RspyDecompile(ScriptPath)
    -- Send dummy decompiled code
    local DummyCode = string.format([[
-- Decompiled from: %s
-- This is dummy decompiled code

local Players = game:GetService("Players")
local LocalPlayer = Players.LocalPlayer

print("Hello from %s")

-- More dummy code here
while task.wait(1) do
    print("Still running...")
end
]], ScriptPath, ScriptPath)

    SendRemoteSpyMessage({
        type = 'rspy_decompiled',
        scriptPath = ScriptPath,
        source = DummyCode
    })

    Log(LOG_SUCCESS, 'Sent dummy decompiled code for: ' .. ScriptPath)
end

function RspyGenerateCode(CallId, Name, Path, RemoteType, Direction, Arguments)
    -- Generate dummy code
    local Args = {}
    for I = 1, #Arguments do
        table.insert(Args, Arguments[I].value)
    end
    local ArgsStr = table.concat(Args, ', ')

    local PathParts = {}
    for Part in string.gmatch(Path, '[^.]+') do
        table.insert(PathParts, Part)
    end

    local Service = PathParts[1]
    local RestPath = table.concat(PathParts, '.', 2)

    local Code
    if RemoteType == 'RemoteEvent' then
        local Method = Direction == 'outgoing' and 'FireServer' or 'FireClient'
        local Target = Direction == 'outgoing' and '' or 'player, '
        Code = string.format('game:GetService("%s").%s:%s(%s%s)', Service, RestPath, Method, Target, ArgsStr)
    else
        local Method = Direction == 'outgoing' and 'InvokeServer' or 'InvokeClient'
        local Target = Direction == 'outgoing' and '' or 'player, '
        Code = string.format('local result = game:GetService("%s").%s:%s(%s%s)', Service, RestPath, Method, Target, ArgsStr)
    end

    SendRemoteSpyMessage({
        type = 'rspy_generated_code',
        callId = CallId,
        code = Code
    })

    Log(LOG_SUCCESS, 'Sent dummy generated code for call: ' .. CallId)
end

--/ Main /--
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
