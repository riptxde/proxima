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

--/ Utilities /--
local function EscapeStringLiteral(Name)
    -- Determine which quote style to use based on what's in the string
    local HasSingleQuote = string.find(Name, "'", 1, true) ~= nil
    local HasDoubleQuote = string.find(Name, '"', 1, true) ~= nil

    if HasSingleQuote and not HasDoubleQuote then
        -- Has single quotes only - use double quotes
        return '"' .. Name .. '"'
    elseif HasSingleQuote and HasDoubleQuote then
        -- Has both quotes - use single quotes and escape them with backslash
        local Escaped = string.gsub(Name, '\\', '\\\\')  -- Escape backslashes first
        Escaped = string.gsub(Escaped, "'", "\\'")       -- Escape single quotes
        return "'" .. Escaped .. "'"
    else
        -- Has no quotes or only double quotes - use single quotes (preferred)
        return "'" .. Name .. "'"
    end
end

local function EscapeInstanceName(Name)
    -- Check if name is a valid Lua identifier
    -- Must start with letter or underscore, followed only by letters, numbers, or underscores
    -- Pattern: ^[A-Za-z_][A-Za-z0-9_]*$
    local IsValidIdentifier = true

    -- Check first character (must be letter or underscore)
    local FirstChar = string.sub(Name, 1, 1)
    if not string.match(FirstChar, '[A-Za-z_]') then
        IsValidIdentifier = false
    end

    -- Check remaining characters (must be letter, number, or underscore)
    if IsValidIdentifier and #Name > 1 then
        for i = 2, #Name do
            local Char = string.sub(Name, i, i)
            if not string.match(Char, '[A-Za-z0-9_]') then
                IsValidIdentifier = false
                break
            end
        end
    end

    if IsValidIdentifier then
        -- Can use dot notation (e.g., Workspace.Part)
        return '.' .. Name
    else
        -- Need bracket notation with escaped string
        return '[' .. EscapeStringLiteral(Name) .. ']'
    end
end

local function BuildInstancePath(Instance)
    -- Build a Lua path string for the instance
    local PathParts = {}
    local Current = Instance

    -- Build path from instance up to game
    while Current and Current ~= game do
        table.insert(PathParts, 1, EscapeInstanceName(Current.Name))
        Current = Current.Parent
    end

    -- Determine root
    local PathRoot = 'game'

    if #PathParts > 0 then
        -- Check if the top-level parent is a direct child of game
        local TopParent = Instance
        while TopParent.Parent and TopParent.Parent ~= game do
            TopParent = TopParent.Parent
        end

        if TopParent.Parent == game then
            -- Check if it's Workspace
            if TopParent:IsA('Workspace') then
                PathRoot = 'workspace'
                table.remove(PathParts, 1) -- Remove Workspace from parts
            else
                -- Check if it's a service
                local Success, Service = pcall(function()
                    return game:GetService(TopParent.ClassName)
                end)

                if Success and Service == TopParent then
                    PathRoot = 'game:GetService(' .. EscapeStringLiteral(TopParent.ClassName) .. ')'
                    table.remove(PathParts, 1) -- Remove service name from parts
                end
            end
        end
    end

    -- Build final path
    local PathString = PathRoot
    if #PathParts > 0 then
        PathString = PathRoot .. table.concat(PathParts, '')
    end

    return PathString
end

local function SendMessage(Type, Data)
    if not Socket then
        return
    end

    Data = Data or {}
    Data.type = Type

    pcall(function()
        local Json = HttpService:JSONEncode(Data)
        Socket:Send(Json)
    end)
end

--/ Functions /--
local function Log(Level, ...)
    local Args = {...}
    for i = 1, #Args do
        Args[i] = tostring(Args[i])
    end
    local Message = table.concat(Args, ' ')

    SendMessage('log', {
        level = Level,
        message = Message
    })
end

local function Ready()
    SendMessage('ready')
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

    SendMessage('register', {
        username = Username
    })
end

local function Pong()
    SendMessage('pong')
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
        RspyGenerateCode(Data.callId)
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
        SendMessage('exp_tree_changed')
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

    SendMessage('exp_tree', {
        nodes = Tree
    })
end

function ExpGetProperties(Id, Properties, SpecialProperties)
    if not ExplorerActive then
        return
    end

    local Instance = GetInstance(Id)

    if not Instance then
        SendMessage('error', {
            request = 'get_explorer_properties',
            message = 'Invalid ID'
        })
        return
    end

    local Props = {}
    local SpecialProps = {}

    -- Build the instance path once for all properties
    local InstancePath = BuildInstancePath(Instance)

    -- Check if gethiddenproperty is available for special properties
    local HasGetHiddenProperty = type(gethiddenproperty) == 'function'

    -- Get regular properties
    for _, PropMetadata in ipairs(Properties) do
        local PropName = PropMetadata.name
        local Success, Value = pcall(function()
            return Instance[PropName]
        end)

        if Success then
            local TypeStr = typeof(Value)
            local ValueStr = tostring(Value)
            local PropertyCode

            -- Check if the value is unreadable (only if property type isn't string)
            if PropMetadata.valueType ~= 'string' and ValueStr:match('^Unable to get property .+, type %S+$') then
                local TypeName = ValueStr:match('type (%S+)$')
                TypeStr = TypeName or PropMetadata.valueType or 'Unknown'
                ValueStr = '[Unreadable]'
                PropertyCode = '-- Property is unreadable'
            elseif Value == nil then
                TypeStr = PropMetadata.valueType or TypeStr
                ValueStr = 'nil'
                PropertyCode = string.format([[-- Get the instance
local instance = %s

-- Get the property value
local value = instance.%s

-- Set the property value
instance.%s = value]], InstancePath, PropName, PropName)
            else
                if TypeStr == 'Instance' then
                    ValueStr = BuildInstancePath(Value)
                elseif TypeStr == 'string' then
                    local TestJson = pcall(function()
                        HttpService:JSONEncode({test = ValueStr})
                    end)
                    if not TestJson then
                        ValueStr = '[Binary/Non-UTF8 data, length: ' .. #ValueStr .. ']'
                    end
                end

                PropertyCode = string.format([[-- Get the instance
local instance = %s

-- Get the property value
local value = instance.%s

-- Set the property value
instance.%s = value]], InstancePath, PropName, PropName)
            end

            Props[PropName] = {
                value = ValueStr,
                type = TypeStr,
                class = Instance.ClassName,
                deprecated = PropMetadata.deprecated,
                hidden = PropMetadata.hidden,
                notScriptable = PropMetadata.not_scriptable,
                pathString = InstancePath,
                propertyCode = PropertyCode
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
                local TypeStr = typeof(Value)
                local ValueStr = tostring(Value)

                -- Check if the value is unreadable (only if property type isn't string)
                if PropMetadata.valueType ~= 'string' and ValueStr:match('^Unable to get property .+, type %S+$') then
                    local TypeName = ValueStr:match('type (%S+)$')
                    TypeStr = TypeName or PropMetadata.valueType or 'Unknown'
                    ValueStr = '[Unreadable]'
                elseif Value == nil then
                    TypeStr = PropMetadata.valueType or TypeStr
                    ValueStr = 'nil'
                else
                    -- For certain types, provide more useful representations
                    if TypeStr == 'Instance' then
                        ValueStr = BuildInstancePath(Value)
                    elseif TypeStr == 'string' then
                        local TestJson = pcall(function()
                            HttpService:JSONEncode({test = ValueStr})
                        end)
                        if not TestJson then
                            ValueStr = '[Binary/Non-UTF8 data, length: ' .. #ValueStr .. ']'
                        end
                    end
                end

                local PropertyCode = string.format([[-- Get the instance
local instance = %s

-- Get the property value
local value = gethiddenproperty(instance, %s)

-- Set the property value
sethiddenproperty(instance, %s, value)]], InstancePath, EscapeStringLiteral(PropName), EscapeStringLiteral(PropName))

                SpecialProps[PropName] = {
                    value = ValueStr,
                    type = TypeStr,
                    class = Instance.ClassName,
                    deprecated = PropMetadata.deprecated,
                    hidden = PropMetadata.hidden,
                    notScriptable = PropMetadata.not_scriptable,
                    pathString = InstancePath,
                    propertyCode = PropertyCode
                }
            end
        end
    end

    SendMessage('exp_properties', {
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
                local PathRoot = 'game'
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
                    if RootInstance:IsA('Workspace') then
                        PathRoot = 'workspace'
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
                            PathRoot = 'game:GetService(' .. EscapeStringLiteral(RootInstance.ClassName) .. ')'
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
                    PathString = PathRoot .. table.concat(PathStringParts, '')
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

    SendMessage('exp_search_results', {
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
        SendMessage('exp_decompiled', {
            id = Id,
            source = '-- Instance not found'
        })
        return
    end

    -- Check if instance is a script type
    if not (Instance:IsA('LocalScript') or Instance:IsA('ModuleScript')) then
        SendMessage('exp_decompiled', {
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

    SendMessage('exp_decompiled', {
        id = Id,
        source = DecompiledSource
    })
end

--/ Remote Spy /--
local RemoteSpyActive = false
local RspyNextCallId = 1
local RspyRemoteToId = {}
local RspyNextRemoteId = 1
local RspyHooks = {}
local RspyConnections = {}
local RspyLogConnectionFunctions = {}
local RspyOriginalCallbacks = {}
local RspyDetouredCallbacks = {}

-- Helper: Get or create a unique ID for a remote instance
local function GetOrCreateRemoteId(RemoteInstance)
    if RspyRemoteToId[RemoteInstance] then
        return RspyRemoteToId[RemoteInstance]
    end

    local RemoteId = RspyNextRemoteId
    RspyNextRemoteId = RspyNextRemoteId + 1
    RspyRemoteToId[RemoteInstance] = RemoteId
    return RemoteId
end

-- Helper: Serialize arguments to a table
local function SerializeArguments(Args, StartIndex)
    local Arguments = {}
    StartIndex = StartIndex or 1

    for i = StartIndex, #Args do
        table.insert(Arguments, {
            type = typeof(Args[i]),
            value = tostring(Args[i])
        })
    end

    return Arguments
end

-- Helper: Get calling script info
local function GetCallingScriptInfo()
    local CallingScript = getcallingscript()

    if CallingScript and typeof(CallingScript) == 'Instance' then
        return BuildInstancePath(CallingScript), CallingScript.Name
    end

    return nil, nil
end

-- Helper: Log a remote call
local function LogRemoteCall(Instance, ClassName, Direction, Arguments, ReturnValue, CallingScriptPath, CallingScriptName)
    local CallId = RspyNextCallId
    RspyNextCallId = RspyNextCallId + 1

    local RemoteId = GetOrCreateRemoteId(Instance)

    SendMessage('rspy_call', {
        callId = CallId,
        remoteId = RemoteId,
        name = Instance.Name,
        path = BuildInstancePath(Instance),
        class = ClassName,
        direction = Direction,
        timestamp = os.date('!%Y-%m-%dT%H:%M:%S') .. '.000Z',
        arguments = Arguments,
        returnValue = ReturnValue,
        callingScriptName = CallingScriptName,
        callingScriptPath = CallingScriptPath,
    })
end

-- Incoming: Create connection function for RemoteEvent/UnreliableRemoteEvent
local function RspyCreateConnectionFunction(Instance, ClassName)
    local ConnectionFunction = function(...)
        local Arguments = SerializeArguments({...})
        LogRemoteCall(Instance, ClassName, 'incoming', Arguments, nil, nil, nil)
    end

    RspyLogConnectionFunctions[ConnectionFunction] = true
    return ConnectionFunction
end

-- Incoming: Create callback detour for RemoteFunction
local function RspyCreateCallbackDetour(Instance, ClassName, Callback)
    local Detour = function(...)
        local Arguments = SerializeArguments({...})
        local Result = table.pack(Callback(...))

        local ReturnValue = nil
        if Result.n > 0 and Result[1] ~= nil then
            ReturnValue = {
                type = typeof(Result[1]),
                value = tostring(Result[1])
            }
        end

        LogRemoteCall(Instance, ClassName, 'incoming', Arguments, ReturnValue, nil, nil)
        return table.unpack(Result)
    end

    return Detour
end

-- Incoming: Setup hooks for an instance
local function RspyHandleInstance(Instance)
    local ClassName = Instance.ClassName

    if ClassName == 'RemoteEvent' or ClassName == 'UnreliableRemoteEvent' then
        -- Hook OnClientEvent signal
        local Connection = Instance.OnClientEvent:Connect(RspyCreateConnectionFunction(Instance, ClassName))
        table.insert(RspyConnections, Connection)

    elseif ClassName == 'RemoteFunction' and getcallbackvalue then
        -- For existing callbacks, store and re-assign to trigger __newindex hook
        local Success, Callback = pcall(getcallbackvalue, Instance, 'OnClientInvoke')
        if Success and typeof(Callback) == 'function' then
            RspyOriginalCallbacks[Instance] = Callback
            Instance.OnClientInvoke = Callback
        end
    end
end

-- Outgoing: Setup hooks
local function RspySetupOutgoingHooks()
    -- Hook __namecall to catch remote:FireServer() and remote:InvokeServer() calls
    local OriginalNamecall
    OriginalNamecall = hookmetamethod(game, '__namecall', function(...)
        local self = ...
        local method = getnamecallmethod()
        local ClassName = typeof(self) == 'Instance' and self.ClassName or nil

        local ShouldLog = (method == 'FireServer' and (ClassName == 'RemoteEvent' or ClassName == 'UnreliableRemoteEvent'))
            or (method == 'InvokeServer' and ClassName == 'RemoteFunction')

        if ShouldLog then
            local Args = {...}
            local Result = table.pack(OriginalNamecall(...))

            local CallingScriptPath, CallingScriptName = GetCallingScriptInfo()
            local Arguments = SerializeArguments(Args, 2)

            local ReturnValue = nil
            if ClassName == 'RemoteFunction' and Result.n > 0 and Result[1] ~= nil then
                ReturnValue = {type = typeof(Result[1]), value = tostring(Result[1])}
            end

            LogRemoteCall(self, ClassName, 'outgoing', Arguments, ReturnValue, CallingScriptPath, CallingScriptName)
            return table.unpack(Result)
        end

        return OriginalNamecall(...)
    end)
    RspyHooks.Namecall = OriginalNamecall

    -- Hook direct function calls (e.g., remote.FireServer(...) instead of remote:FireServer(...))
    local function CreateIndexedHook(ClassName, MethodName, HookName)
        local Prototype = Instance.new(ClassName)
        local OriginalMethod = Prototype[MethodName]

        RspyHooks[HookName] = hookfunction(OriginalMethod, function(self, ...)
            local Args = {...}
            local Result = table.pack(RspyHooks[HookName](self, ...))

            if typeof(self) == 'Instance' and self.ClassName == ClassName then
                local CallingScriptPath, CallingScriptName = GetCallingScriptInfo()
                local Arguments = SerializeArguments(Args)

                local ReturnValue = nil
                if ClassName == 'RemoteFunction' and Result.n > 0 and Result[1] ~= nil then
                    ReturnValue = {type = typeof(Result[1]), value = tostring(Result[1])}
                end

                LogRemoteCall(self, ClassName, 'outgoing', Arguments, ReturnValue, CallingScriptPath, CallingScriptName)
            end

            return table.unpack(Result)
        end)
    end

    CreateIndexedHook('RemoteEvent', 'FireServer', 'FireServer')
    CreateIndexedHook('UnreliableRemoteEvent', 'FireServer', 'UnreliableFireServer')
    CreateIndexedHook('RemoteFunction', 'InvokeServer', 'InvokeServer')
end

-- Incoming: Setup metamethod hooks
local function RspySetupIncomingHooks()
    -- Hook __newindex to catch OnClientInvoke assignments
    local OriginalNewIndex
    OriginalNewIndex = hookmetamethod(game, '__newindex', function(self, key, value)
        if typeof(self) == 'Instance' and self.ClassName == 'RemoteFunction' then
            if key == 'OnClientInvoke' and typeof(value) == 'function' then
                local Detour = RspyCreateCallbackDetour(self, 'RemoteFunction', value)
                RspyDetouredCallbacks[self] = {original = value, detour = Detour}
                return OriginalNewIndex(self, key, Detour)
            end
        end

        return OriginalNewIndex(self, key, value)
    end)
    RspyHooks.NewIndex = OriginalNewIndex
end

function RspyStart()
    if RemoteSpyActive then
        return
    end

    RemoteSpyActive = true

    -- Setup metamethod hooks
    RspySetupOutgoingHooks()
    RspySetupIncomingHooks()

    -- Watch for new remote instances
    local DescendantAddedConnection = game.DescendantAdded:Connect(function(descendant)
        if descendant.ClassName == 'RemoteEvent' or descendant.ClassName == 'UnreliableRemoteEvent' or descendant.ClassName == 'RemoteFunction' then
            RspyHandleInstance(descendant)
        end
    end)
    table.insert(RspyConnections, DescendantAddedConnection)

    -- Hook all existing remote instances
    for _, descendant in ipairs(game:GetDescendants()) do
        if descendant.ClassName == 'RemoteEvent' or descendant.ClassName == 'UnreliableRemoteEvent' or descendant.ClassName == 'RemoteFunction' then
            RspyHandleInstance(descendant)
        end
    end

    Log(LOG_SUCCESS, 'Remote spy started')
end

function RspyStop()
    if not RemoteSpyActive then
        return
    end

    RemoteSpyActive = false

    -- Restore all hooks
    if RspyHooks.Namecall then
        hookmetamethod(game, '__namecall', RspyHooks.Namecall)
    end
    if RspyHooks.NewIndex then
        hookmetamethod(game, '__newindex', RspyHooks.NewIndex)
    end
    if RspyHooks.FireServer then
        hookfunction(Instance.new('RemoteEvent').FireServer, RspyHooks.FireServer)
    end
    if RspyHooks.UnreliableFireServer then
        hookfunction(Instance.new('UnreliableRemoteEvent').FireServer, RspyHooks.UnreliableFireServer)
    end
    if RspyHooks.InvokeServer then
        hookfunction(Instance.new('RemoteFunction').InvokeServer, RspyHooks.InvokeServer)
    end

    -- Disconnect all event connections
    for _, connection in ipairs(RspyConnections) do
        if connection and connection.Connected then
            connection:Disconnect()
        end
    end

    -- Restore RemoteFunction callbacks
    for instance, originalCallback in pairs(RspyOriginalCallbacks) do
        if instance and typeof(instance) == 'Instance' then
            instance.OnClientInvoke = originalCallback
        end
    end
    for instance, callbacks in pairs(RspyDetouredCallbacks) do
        if instance and typeof(instance) == 'Instance' and not RspyOriginalCallbacks[instance] then
            instance.OnClientInvoke = callbacks.original
        end
    end

    -- Clear all session state
    RspyNextCallId = 1
    RspyRemoteToId = {}
    RspyNextRemoteId = 1
    RspyHooks = {}
    RspyConnections = {}
    RspyLogConnectionFunctions = {}
    RspyOriginalCallbacks = {}
    RspyDetouredCallbacks = {}

    Log(LOG_SUCCESS, 'Remote spy stopped')
end

function RspyDecompile(ScriptPath)
    SendMessage('rspy_decompiled', {
        scriptPath = ScriptPath,
        source = '-- Decompile not implemented'
    })
end

function RspyGenerateCode(CallId)
    SendMessage('rspy_generated_code', {
        callId = CallId,
        code = '-- Code generation not implemented'
    })
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
