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
    elseif Data.type == 'start_explorer' then
        HandleStartExplorer()
    elseif Data.type == 'stop_explorer' then
        HandleStopExplorer()
    elseif Data.type == 'get_explorer_tree' then
        HandleGetExplorerTree(Data.expandedIds or {})
    elseif Data.type == 'get_explorer_properties' then
        HandleGetExplorerProperties(Data.id, Data.properties or {}, Data.specialProperties or {})
    elseif Data.type == 'search_explorer' then
        HandleSearchExplorer(Data.query, Data.searchIn, Data.maxResults or 1000)
    end
end

--/ Explorer /--
-- Explorer State
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
            type = 'tree_changed'
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

function HandleStartExplorer()
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

function HandleStopExplorer()
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

function HandleGetExplorerTree(ExpandedIds)
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
        type = 'tree',
        nodes = Tree
    })
end

function HandleGetExplorerProperties(Id, Properties, SpecialProperties)
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
                notScriptable = PropMetadata.not_scriptable
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
                    notScriptable = PropMetadata.not_scriptable
                }
            end
        end
    end

    SendExplorerMessage({
        type = 'properties',
        id = Id,
        props = Props,
        specialProps = SpecialProps
    })
end

function HandleSearchExplorer(Query, SearchIn, MaxResults)
    if not ExplorerActive then
        return
    end

    local Results = {}
    local Descendants = game:GetDescendants()
    local Count = 0
    local Limited = false
    local QueryLower = string.lower(Query)

    for I = 1, #Descendants do
        local Descendant = Descendants[I]
        local Matches = false

        if SearchIn == 'name' or SearchIn == 'both' then
            if string.find(string.lower(Descendant.Name), QueryLower, 1, true) then
                Matches = true
            end
        end

        if (SearchIn == 'class' or SearchIn == 'both') and not Matches then
            if string.find(string.lower(Descendant.ClassName), QueryLower, 1, true) then
                Matches = true
            end
        end

        if Matches then
            Count = Count + 1

            if Count <= MaxResults then
                local PathParts = {}
                local Current = Descendant.Parent

                while Current and Current ~= game do
                    local Id = GetId(Current)
                    if Id then
                        table.insert(PathParts, 1, Id)
                    end
                    Current = Current.Parent
                end

                local Children = Descendant:GetChildren()

                table.insert(Results, {
                    id = GetOrCreateId(Descendant),
                    n = Descendant.Name,
                    c = Descendant.ClassName,
                    path = PathParts,
                    h = #Children > 0
                })
            else
                Limited = true
            end
        end
    end

    SendExplorerMessage({
        type = 'search_results',
        query = Query,
        results = Results,
        total = Count,
        limited = Limited
    })
end

--/ END EXPLORER SECTION /--

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
