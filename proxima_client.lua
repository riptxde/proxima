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

-- Capabilities
local Capabilities = {
    hookmetamethod = typeof(hookmetamethod) == 'function',
    hookfunction = typeof(hookfunction) == 'function',
    getnamecallmethod = typeof(getnamecallmethod) == 'function',
    getcallbackvalue = typeof(getcallbackvalue) == 'function',
    newcclosure = typeof(newcclosure) == 'function',
    getcallingscript = typeof(getcallingscript) == 'function',
    decompile = typeof(decompile) == 'function',
    gethiddenproperty = typeof(gethiddenproperty) == 'function',
}

--/ Utilities /--
local function EscapeStringLiteral(Name)
    -- Escape special characters
    local EscapeMap = {
        ['\\'] = '\\\\',
        ['\n'] = '\\n',
        ['\r'] = '\\r',
        ['\t'] = '\\t',
        ['\a'] = '\\a',
        ['\b'] = '\\b',
        ['\v'] = '\\v',
        ['\f'] = '\\f',
    }

    -- Determine which quote style to use based on what's in the string
    local HasSingleQuote = string.find(Name, "'", 1, true) ~= nil
    local HasDoubleQuote = string.find(Name, '"', 1, true) ~= nil

    if HasSingleQuote and not HasDoubleQuote then
        -- Has single quotes only - use double quotes
        local Escaped = string.gsub(Name, '[\\%c]', function(c)
            if c == '"' then return '\\"' end
            return EscapeMap[c] or string.format('\\x%02X', string.byte(c))
        end)
        return '"' .. Escaped .. '"'
    elseif HasSingleQuote and HasDoubleQuote then
        -- Has both quotes - use single quotes and escape them
        local Escaped = string.gsub(Name, '[\\%c\']', function(c)
            if c == "'" then return "\\'" end
            return EscapeMap[c] or string.format('\\x%02X', string.byte(c))
        end)
        return "'" .. Escaped .. "'"
    else
        -- Has no quotes or only double quotes - use single quotes (preferred)
        local Escaped = string.gsub(Name, '[\\%c]', function(c)
            return EscapeMap[c] or string.format('\\x%02X', string.byte(c))
        end)
        return "'" .. Escaped .. "'"
    end
end

local function IsValidIdentifier(Str)
    -- Check if string is a valid Lua identifier
    return type(Str) == 'string' and Str:match('^[A-Za-z_][A-Za-z0-9_]*$') ~= nil
end

local function EscapeInstanceName(Name)
    if IsValidIdentifier(Name) then
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

local function TryCommaSeparatedConstructor(ValueType, ToStringValue)
    -- Check if tostring value is comma-separated numbers that can use ClassName.new(tostring)
    -- e.g., "1, 2, 3" for Vector3 -> Vector3.new(1, 2, 3)
    local Pattern = '^%-?%d+%.?%d*'  -- Number pattern
    local Parts = {}

    for Part in string.gmatch(ToStringValue, '[^,]+') do
        local Trimmed = string.match(Part, '^%s*(.-)%s*$')  -- Trim whitespace
        if string.match(Trimmed, Pattern .. '$') then
            table.insert(Parts, Trimmed)
        else
            return nil  -- Not all parts are numbers
        end
    end

    if #Parts > 0 then
        return ValueType .. '.new(' .. table.concat(Parts, ', ') .. ')'
    end

    return nil
end

local function Round(Num, Decimals)
    -- Round a number to n decimal places (default 3)
    Decimals = Decimals or 3
    local Mult = 10 ^ Decimals
    return math.floor(Num * Mult + 0.5) / Mult
end

local function FormatNumber(Num, Decimals)
    -- Format number to n decimal places (default 3), removing trailing zeros
    Decimals = Decimals or 3
    local Rounded = Round(Num, Decimals)

    if Rounded == math.floor(Rounded) then
        -- Integer, no decimal needed
        return tostring(Rounded)
    end

    local Formatted = string.format('%.' .. Decimals .. 'f', Rounded)
    -- Remove trailing zeros after decimal point
    Formatted = string.gsub(Formatted, '%.?0+$', '')
    return Formatted
end

local function Serialize(Value, Depth, Shown)
    -- Serialize Lua values to readable code
    Depth = Depth or 0
    Shown = Shown or {}

    local Indent = '    '
    local Tabs = Indent:rep(Depth)

    -- Primitives
    if type(Value) == 'nil' then
        return 'nil'
    elseif type(Value) == 'boolean' then
        return tostring(Value)
    elseif type(Value) == 'number' then
        if Value == math.huge then
            return 'math.huge'
        elseif Value == -math.huge then
            return '-math.huge'
        elseif Value == math.pi then
            return 'math.pi'
        elseif Value ~= Value then
            -- NaN detection
            return '0/0'
        end
        return tostring(Value)
    elseif type(Value) == 'string' then
        return EscapeStringLiteral(Value)
    end

    -- Tables with __tostring
    if type(Value) == 'table' then
        local Meta = getmetatable(Value)
        if Meta and type(Meta.__tostring) == 'function' then
            return tostring(Value)
        elseif type(Value.__tostring) == 'function' then
            return tostring(Value)
        end
    end

    -- Tables
    if type(Value) == 'table' then
        if Shown[Value] then
            return '{--[[ Cyclic Table ]]--}' -- You can't send cyclic tables over remotes anyway so like, this should never happen
        end
        Shown[Value] = true

        -- Check if array
        local IsArray = true
        for Key in next, Value do
            if type(Key) ~= 'number' then
                IsArray = false
                break
            end
        end

        local Result = '{\n' .. Indent .. Tabs

        if IsArray then
            for Index = 1, #Value do
                if Index > 1 then
                    Result = Result .. ',\n' .. Indent .. Tabs
                end
                Result = Result .. Serialize(Value[Index], Depth + 1, Shown)
            end
        else
            local Keys = {}
            local KeyValues = {}

            for Key, Val in next, Value do
                local KeyStr
                if IsValidIdentifier(Key) then
                    KeyStr = Key
                else
                    KeyStr = '[' .. Serialize(Key, Depth + 1, Shown) .. ']'
                end
                table.insert(Keys, KeyStr)
                KeyValues[KeyStr] = Serialize(Val, Depth + 1, Shown)
            end

            table.sort(Keys)

            for Index, KeyStr in ipairs(Keys) do
                if Index > 1 then
                    Result = Result .. ',\n' .. Indent .. Tabs
                end
                Result = Result .. KeyStr .. ' = ' .. KeyValues[KeyStr]
            end
        end

        Shown[Value] = false
        Result = Result .. '\n' .. Tabs .. '}'
        return Result
    end

    -- Roblox types
    if not typeof then
        return ('--[[ %s ]]--'):format(type(Value))
    end

    local ValueType = typeof(Value)

    if ValueType == 'Instance' then
        return BuildInstancePath(Value)
    elseif ValueType == 'Axes' then
        local Components = {}
        if Value.X then table.insert(Components, 'Enum.Axis.X') end
        if Value.Y then table.insert(Components, 'Enum.Axis.Y') end
        if Value.Z then table.insert(Components, 'Enum.Axis.Z') end
        return 'Axes.new(' .. table.concat(Components, ', ') .. ')'
    elseif ValueType == 'BrickColor' then
        return 'BrickColor.new(' .. ('%q'):format(Value.Name) .. ')'
    elseif ValueType == 'CFrame' then
        -- Round components to 3 decimal places and check against identity
        local Components = table.pack(Value:GetComponents())
        for i = 1, Components.n do
            Components[i] = Round(Components[i])
        end
        local Rounded = CFrame.new(table.unpack(Components, 1, Components.n))

        if Rounded == CFrame.identity then
            return 'CFrame.identity'
        end
        -- Use position + rotation for more intuitive representation
        local Position = Value.Position
        local X, Y, Z = Value:ToOrientation()

        -- Format each angle individually - use degrees with math.rad() if non-zero
        local function FormatAngle(Radians)
            if math.abs(Round(Radians)) > 0 then
                -- Non-zero: use degrees with math.rad()
                return 'math.rad(' .. FormatNumber(math.deg(Radians)) .. ')'
            else
                -- Zero: just use 0
                return '0'
            end
        end

        return ('CFrame.new(%s, %s, %s) * CFrame.Angles(%s, %s, %s)'):format(
            FormatNumber(Position.X), FormatNumber(Position.Y), FormatNumber(Position.Z),
            FormatAngle(X), FormatAngle(Y), FormatAngle(Z)
        )
    elseif ValueType == 'Color3' then
        return ('Color3.new(%s, %s, %s)'):format(FormatNumber(Value.R), FormatNumber(Value.G), FormatNumber(Value.B))
    elseif ValueType == 'ColorSequence' then
        if #Value.Keypoints > 2 then
            return 'ColorSequence.new(' .. Serialize(Value.Keypoints, Depth, Shown) .. ')'
        elseif Value.Keypoints[1].Value == Value.Keypoints[2].Value then
            return 'ColorSequence.new(' .. Serialize(Value.Keypoints[1].Value, Depth, Shown) .. ')'
        else
            return 'ColorSequence.new(' .. Serialize(Value.Keypoints[1].Value, Depth, Shown) .. ', ' .. Serialize(Value.Keypoints[2].Value, Depth, Shown) .. ')'
        end
    elseif ValueType == 'ColorSequenceKeypoint' then
        return ('ColorSequenceKeypoint.new(%s, %s)'):format(FormatNumber(Value.Time), Serialize(Value.Value, Depth, Shown))
    elseif ValueType == 'DateTime' then
        return 'DateTime.fromIsoDate(' .. EscapeStringLiteral(Value:ToIsoDate()) .. ')'
    elseif ValueType == 'Enum' then
        return 'Enum.' .. tostring(Value)
    elseif ValueType == 'EnumItem' then
        return 'Enum.' .. tostring(Value.EnumType) .. '.' .. Value.Name
    elseif ValueType == 'Faces' then
        local Components = {}
        for _, Item in ipairs(Enum.NormalId:GetEnumItems()) do
            if Value[Item.Name] then
                table.insert(Components, 'Enum.NormalId.' .. Item.Name)
            end
        end
        return 'Faces.new(' .. table.concat(Components, ', ') .. ')'
    elseif ValueType == 'NumberRange' then
        if Value.Min == Value.Max then
            return ('NumberRange.new(%s)'):format(FormatNumber(Value.Min))
        else
            return ('NumberRange.new(%s, %s)'):format(FormatNumber(Value.Min), FormatNumber(Value.Max))
        end
    elseif ValueType == 'NumberSequence' then
        if #Value.Keypoints > 2 then
            return 'NumberSequence.new(' .. Serialize(Value.Keypoints, Depth, Shown) .. ')'
        elseif Value.Keypoints[1].Value == Value.Keypoints[2].Value then
            return ('NumberSequence.new(%s)'):format(FormatNumber(Value.Keypoints[1].Value))
        else
            return ('NumberSequence.new(%s, %s)'):format(FormatNumber(Value.Keypoints[1].Value), FormatNumber(Value.Keypoints[2].Value))
        end
    elseif ValueType == 'NumberSequenceKeypoint' then
        if Value.Envelope ~= 0 then
            return ('NumberSequenceKeypoint.new(%s, %s, %s)'):format(FormatNumber(Value.Time), FormatNumber(Value.Value), FormatNumber(Value.Envelope))
        else
            return ('NumberSequenceKeypoint.new(%s, %s)'):format(FormatNumber(Value.Time), FormatNumber(Value.Value))
        end
    elseif ValueType == 'PathWaypoint' then
        return 'PathWaypoint.new(' .. Serialize(Value.Position, Depth, Shown) .. ', ' .. Serialize(Value.Action, Depth, Shown) .. ')'
    elseif ValueType == 'PhysicalProperties' then
        return ('PhysicalProperties.new(%s, %s, %s, %s, %s)'):format(
            FormatNumber(Value.Density),
            FormatNumber(Value.Friction),
            FormatNumber(Value.Elasticity),
            FormatNumber(Value.FrictionWeight),
            FormatNumber(Value.ElasticityWeight)
        )
    elseif ValueType == 'Ray' then
        return 'Ray.new(' .. Serialize(Value.Origin, Depth, Shown) .. ', ' .. Serialize(Value.Direction, Depth, Shown) .. ')'
    elseif ValueType == 'Rect' then
        return ('Rect.new(%s, %s, %s, %s)'):format(
            FormatNumber(Value.Min.X),
            FormatNumber(Value.Min.Y),
            FormatNumber(Value.Max.X),
            FormatNumber(Value.Max.Y)
        )
    elseif ValueType == 'Region3' then
        local Min = Value.CFrame.Position + Value.Size * -0.5
        local Max = Value.CFrame.Position + Value.Size * 0.5
        return 'Region3.new(' .. Serialize(Min, Depth, Shown) .. ', ' .. Serialize(Max, Depth, Shown) .. ')'
    elseif ValueType == 'Region3int16' then
        return 'Region3int16.new(' .. Serialize(Value.Min, Depth, Shown) .. ', ' .. Serialize(Value.Max, Depth, Shown) .. ')'
    elseif ValueType == 'TweenInfo' then
        return ('TweenInfo.new(%s, %s, %s, %s, %s, %s)'):format(
            FormatNumber(Value.Time),
            Serialize(Value.EasingStyle, Depth, Shown),
            Serialize(Value.EasingDirection, Depth, Shown),
            Value.RepeatCount,
            Serialize(Value.Reverses, Depth, Shown),
            FormatNumber(Value.DelayTime)
        )
    elseif ValueType == 'UDim' then
        return ('UDim.new(%s, %s)'):format(FormatNumber(Value.Scale), FormatNumber(Value.Offset))
    elseif ValueType == 'UDim2' then
        if Value.X.Offset == 0 and Value.Y.Offset == 0 then
            return ('UDim2.fromScale(%s, %s)'):format(FormatNumber(Value.X.Scale), FormatNumber(Value.Y.Scale))
        elseif Value.X.Scale == 0 and Value.Y.Scale == 0 then
            return ('UDim2.fromOffset(%s, %s)'):format(FormatNumber(Value.X.Offset), FormatNumber(Value.Y.Offset))
        end
        return ('UDim2.new(%s, %s, %s, %s)'):format(FormatNumber(Value.X.Scale), FormatNumber(Value.X.Offset), FormatNumber(Value.Y.Scale), FormatNumber(Value.Y.Offset))
    elseif ValueType == 'Vector2' then
        -- Round components to 3 decimal places and check against constants
        local Rounded = Vector2.new(Round(Value.X), Round(Value.Y))

        if Rounded == Vector2.zero then
            return 'Vector2.zero'
        elseif Rounded == Vector2.one then
            return 'Vector2.one'
        elseif Rounded == Vector2.xAxis then
            return 'Vector2.xAxis'
        elseif Rounded == Vector2.yAxis then
            return 'Vector2.yAxis'
        end
        return ('Vector2.new(%s, %s)'):format(FormatNumber(Value.X), FormatNumber(Value.Y))
    elseif ValueType == 'Vector2int16' then
        return ('Vector2int16.new(%s, %s)'):format(Value.X, Value.Y)
    elseif ValueType == 'Vector3' then
        -- Round components to 3 decimal places and check against constants
        local Rounded = Vector3.new(Round(Value.X), Round(Value.Y), Round(Value.Z))

        if Rounded == Vector3.zero then
            return 'Vector3.zero'
        elseif Rounded == Vector3.one then
            return 'Vector3.one'
        elseif Rounded == Vector3.xAxis then
            return 'Vector3.xAxis'
        elseif Rounded == Vector3.yAxis then
            return 'Vector3.yAxis'
        elseif Rounded == Vector3.zAxis then
            return 'Vector3.zAxis'
        end
        return ('Vector3.new(%s, %s, %s)'):format(FormatNumber(Value.X), FormatNumber(Value.Y), FormatNumber(Value.Z))
    elseif ValueType == 'Vector3int16' then
        return ('Vector3int16.new(%s, %s, %s)'):format(Value.X, Value.Y, Value.Z)
    elseif ValueType == 'Font' then
        return 'Font.new(' .. Serialize(Value.Family, Depth, Shown) .. ', ' .. Serialize(Value.Weight, Depth, Shown) .. ', ' .. Serialize(Value.Style, Depth, Shown) .. ')'
    elseif ValueType == 'FloatCurveKey' then
        return ('FloatCurveKey.new(%s, %s, %s)'):format(FormatNumber(Value.Time), FormatNumber(Value.Value), Serialize(Value.Interpolation, Depth, Shown))
    elseif ValueType == 'RotationCurveKey' then
        return 'RotationCurveKey.new(' .. FormatNumber(Value.Time) .. ', ' .. Serialize(Value.Value, Depth, Shown) .. ', ' .. Serialize(Value.Interpolation, Depth, Shown) .. ')'
    elseif ValueType == 'Random' then
        return 'Random.new()'
    elseif ValueType == 'buffer' then
        local Encoded = HttpService:JSONEncode(Value)
        return 'game:GetService("HttpService"):JSONDecode(' .. EscapeStringLiteral(Encoded) .. ')'
    elseif ValueType == 'userdata' then
        if getmetatable(Value) ~= nil then
            return 'newproxy(true)'
        else
            return 'newproxy()'
        end
    else
        -- Try comma-separated constructor as fallback for unknown types
        local ToStringValue = tostring(Value)
        local CommaSeparatedResult = TryCommaSeparatedConstructor(ValueType, ToStringValue)
        if CommaSeparatedResult then
            return CommaSeparatedResult
        end

        return ('--[[ %s ]]--'):format(ValueType)
    end
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
        RspyDecompile(Data.callId)
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
            local SerializedValue

            -- Check if the value is unreadable (only if property type isn't string)
            if PropMetadata.valueType ~= 'string' and ValueStr:match('^Unable to get property .+, type %S+$') then
                local TypeName = ValueStr:match('type (%S+)$')
                TypeStr = TypeName or PropMetadata.valueType or 'Unknown'
                ValueStr = '[Unreadable]'
                PropertyCode = '-- Property is unreadable'
            elseif Value == nil then
                TypeStr = PropMetadata.valueType or TypeStr
                ValueStr = 'nil'
                SerializedValue = 'nil'
            else
                -- Check if string is binary/non-UTF8 before serializing
                local IsBinaryString = false
                if TypeStr == 'string' then
                    local TestJson = pcall(function()
                        HttpService:JSONEncode({test = ValueStr})
                    end)
                    if not TestJson then
                        IsBinaryString = true
                        ValueStr = '[Binary/Non-UTF8 data, length: ' .. #ValueStr .. ']'
                    end
                end

                -- Serialize for both display and property code (skip if binary)
                if IsBinaryString then
                    SerializedValue = "'[Binary data]'"
                else
                    local SerializeSuccess, Result = pcall(Serialize, Value)
                    if SerializeSuccess then
                        SerializedValue = Result
                        ValueStr = Result  -- Use serialized value for display too
                    else
                        SerializedValue = 'nil'
                        ValueStr = tostring(Value)  -- Fallback to tostring
                    end
                end
            end

            -- Build property code
            if PropertyCode ~= '-- Property is unreadable' then
                PropertyCode = ([[-- Get the instance
local instance = %s

-- Get the property value
local value = instance.%s

-- Set the property value
instance.%s = %s]]):format(InstancePath, PropName, PropName, SerializedValue)
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
    if Capabilities.gethiddenproperty then
        for _, PropMetadata in ipairs(SpecialProperties) do
            local PropName = PropMetadata.name
            local Success, Value = pcall(function()
                return gethiddenproperty(Instance, PropName)
            end)

            if Success then
                local TypeStr = typeof(Value)
                local ValueStr = tostring(Value)
                local SerializedValue

                -- Check if the value is unreadable (only if property type isn't string)
                if PropMetadata.valueType ~= 'string' and ValueStr:match('^Unable to get property .+, type %S+$') then
                    local TypeName = ValueStr:match('type (%S+)$')
                    TypeStr = TypeName or PropMetadata.valueType or 'Unknown'
                    ValueStr = '[Unreadable]'
                    SerializedValue = 'nil'
                elseif Value == nil then
                    TypeStr = PropMetadata.valueType or TypeStr
                    ValueStr = 'nil'
                    SerializedValue = 'nil'
                else
                    -- Check if string is binary/non-UTF8 before serializing
                    local IsBinaryString = false
                    if TypeStr == 'string' then
                        local TestJson = pcall(function()
                            HttpService:JSONEncode({test = ValueStr})
                        end)
                        if not TestJson then
                            IsBinaryString = true
                            ValueStr = '[Binary/Non-UTF8 data, length: ' .. #ValueStr .. ']'
                        end
                    end

                    -- Serialize for both display and property code (skip if binary)
                    if IsBinaryString then
                        SerializedValue = "'[Binary data]'"
                    else
                        local SerializeSuccess, Result = pcall(Serialize, Value)
                        if SerializeSuccess then
                            SerializedValue = Result
                            ValueStr = Result  -- Use serialized value for display too
                        else
                            SerializedValue = 'nil'
                            ValueStr = tostring(Value)  -- Fallback to tostring
                        end
                    end
                end

                local PropertyCode = ([[-- Get the instance
local instance = %s

-- Get the property value
local value = gethiddenproperty(instance, %s)

-- Set the property value
sethiddenproperty(instance, %s, %s)]]):format(InstancePath, EscapeStringLiteral(PropName), EscapeStringLiteral(PropName), SerializedValue)

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
    local DecompiledSource = '-- Your executor does not support script decompilation'

    if Capabilities.decompile then
        local Success, Result = pcall(decompile, Instance)
        if Success then
            DecompiledSource = Result
        else
            DecompiledSource = ('--[[\nError decompiling: %s\n]]--'):format(tostring(Result))
        end
    end

    SendMessage('exp_decompiled', {
        id = Id,
        source = DecompiledSource
    })
end

--/ Remote Spy /--
local RemoteSpyActive = false
local RspyRemoteToId = {}
local RspyNextRemoteId = 1
local RspyCallIdToCallData = {}
local RspyNextCallId = 1
local RspyHooks = {}
local RspyConnections = {}
local RspyLogConnectionFunctions = {}
local RspyDetouredCallbacks = {}

local function GetOrCreateRemoteId(RemoteInstance)
    if RspyRemoteToId[RemoteInstance] then
        return RspyRemoteToId[RemoteInstance]
    end

    local RemoteId = RspyNextRemoteId
    RspyNextRemoteId = RspyNextRemoteId + 1
    RspyRemoteToId[RemoteInstance] = RemoteId
    return RemoteId
end

local function GetCallingScript()
    if not Capabilities.getcallingscript then
        return nil
    end

    local Success, CallingScript = pcall(getcallingscript)

    if Success and CallingScript and typeof(CallingScript) == 'Instance' then
        return CallingScript
    end

    return nil
end

local function LogRemoteCall(Instance, ClassName, Direction, Arguments, ReturnValues, CallingScript)
    local CallId = RspyNextCallId
    RspyNextCallId = RspyNextCallId + 1

    local RemoteId = GetOrCreateRemoteId(Instance)

    local CallingScriptPath = nil
    local CallingScriptName = nil
    if CallingScript then
        CallingScriptPath = BuildInstancePath(CallingScript)
        CallingScriptName = CallingScript.Name
    end

    -- Serialize arguments and return values for display
    local SerializedArguments = {}
    for i = 1, #Arguments do
        table.insert(SerializedArguments, {
            type = typeof(Arguments[i]),
            value = Serialize(Arguments[i])
        })
    end

    local SerializedReturnValues = nil
    if ReturnValues ~= nil and #ReturnValues > 0 then
        SerializedReturnValues = {}
        for i = 1, #ReturnValues do
            table.insert(SerializedReturnValues, {
                type = typeof(ReturnValues[i]),
                value = Serialize(ReturnValues[i])
            })
        end
    end

    -- Store call data for decompile and code generation
    RspyCallIdToCallData[CallId] = {
        instance = Instance,
        className = ClassName,
        direction = Direction,
        arguments = Arguments,
        returnValues = ReturnValues,
        callingScript = CallingScript,
    }

    SendMessage('rspy_call', {
        callId = CallId,
        remoteId = RemoteId,
        name = Instance.Name,
        path = BuildInstancePath(Instance),
        class = ClassName,
        direction = Direction,
        timestamp = os.date('!%Y-%m-%dT%H:%M:%S') .. '.000Z',
        arguments = SerializedArguments,
        returnValues = SerializedReturnValues,
        callingScriptName = CallingScriptName,
        callingScriptPath = CallingScriptPath,
    })
end

-- Incoming: Create connection function for RemoteEvent/UnreliableRemoteEvent
local function RspyCreateConnectionFunction(Instance, ClassName)
    local ConnectionFunction = newcclosure(function(...)
        LogRemoteCall(Instance, ClassName, 'incoming', {...}, nil, nil)
    end)

    RspyLogConnectionFunctions[ConnectionFunction] = true
    return ConnectionFunction
end

-- Incoming: Create callback detour for RemoteFunction
local function RspyCreateCallbackDetour(Instance, ClassName, Callback)
    local Detour = newcclosure(function(...)
        local Result = table.pack(Callback(...))

        local ReturnValues = nil
        if Result.n > 0 then
            ReturnValues = {}
            for i = 1, Result.n do
                table.insert(ReturnValues, Result[i])
            end
        end

        LogRemoteCall(Instance, ClassName, 'incoming', {...}, ReturnValues, nil)
        return table.unpack(Result, 1, Result.n)
    end)

    return Detour
end

-- Incoming: Setup hooks for an instance
local function RspyHandleInstance(Instance)
    local ClassName = Instance.ClassName

    if ClassName == 'RemoteEvent' or ClassName == 'UnreliableRemoteEvent' then
        -- Hook OnClientEvent signal
        local Connection = Instance.OnClientEvent:Connect(RspyCreateConnectionFunction(Instance, ClassName))
        table.insert(RspyConnections, Connection)

    elseif ClassName == 'RemoteFunction' and Capabilities.getcallbackvalue then
        -- For existing callbacks, re-assign to trigger __newindex hook
        local Success, Callback = pcall(getcallbackvalue, Instance, 'OnClientInvoke')
        if Success and typeof(Callback) == 'function' then
            Instance.OnClientInvoke = Callback
        end
    end
end

-- Outgoing: Setup hooks
local function RspySetupOutgoingHooks()
    -- Hook __namecall to catch remote:FireServer() and remote:InvokeServer() calls
    local OriginalNamecall
    OriginalNamecall = hookmetamethod(game, '__namecall', newcclosure(function(...)
        local self = ...
        local method = getnamecallmethod()
        local ClassName = typeof(self) == 'Instance' and self.ClassName or nil

        local ShouldLog = (method == 'FireServer' and (ClassName == 'RemoteEvent' or ClassName == 'UnreliableRemoteEvent'))
            or (method == 'InvokeServer' and ClassName == 'RemoteFunction')

        if ShouldLog then
            local Result = table.pack(OriginalNamecall(...))
            local CallingScript = GetCallingScript()
            local Arguments = table.pack(select(2, ...))

            local ReturnValues = nil
            if ClassName == 'RemoteFunction' and Result.n > 0 then
                ReturnValues = {}
                for i = 1, Result.n do
                    table.insert(ReturnValues, Result[i])
                end
            end

            LogRemoteCall(self, ClassName, 'outgoing', Arguments, ReturnValues, CallingScript)
            return table.unpack(Result, 1, Result.n)
        end

        return OriginalNamecall(...)
    end))
    RspyHooks.Namecall = OriginalNamecall

    -- Hook direct function calls (e.g., remote.FireServer(...) instead of remote:FireServer(...))
    local function CreateIndexedHook(ClassName, MethodName, HookName)
        local Prototype = Instance.new(ClassName)
        local OriginalMethod = Prototype[MethodName]

        RspyHooks[HookName] = hookfunction(OriginalMethod, newcclosure(function(self, ...)
            local Result = table.pack(RspyHooks[HookName](self, ...))

            if typeof(self) == 'Instance' and self.ClassName == ClassName then
                local CallingScript = GetCallingScript()
                local Arguments = table.pack(...)

                local ReturnValues = nil
                if ClassName == 'RemoteFunction' and Result.n > 0 then
                    ReturnValues = {}
                    for i = 1, Result.n do
                        table.insert(ReturnValues, Result[i])
                    end
                end

                LogRemoteCall(self, ClassName, 'outgoing', Arguments, ReturnValues, CallingScript)
            end

            return table.unpack(Result, 1, Result.n)
        end))
    end

    CreateIndexedHook('RemoteEvent', 'FireServer', 'FireServer')
    CreateIndexedHook('UnreliableRemoteEvent', 'FireServer', 'UnreliableFireServer')
    CreateIndexedHook('RemoteFunction', 'InvokeServer', 'InvokeServer')
end

-- Incoming: Setup metamethod hooks
local function RspySetupIncomingHooks()
    -- Hook __newindex to catch OnClientInvoke assignments
    local OriginalNewIndex
    OriginalNewIndex = hookmetamethod(game, '__newindex', newcclosure(function(self, key, value)
        if typeof(self) == 'Instance' and self.ClassName == 'RemoteFunction' then
            if key == 'OnClientInvoke' and typeof(value) == 'function' then
                local Detour = RspyCreateCallbackDetour(self, 'RemoteFunction', value)
                RspyDetouredCallbacks[self] = {original = value, detour = Detour}
                return OriginalNewIndex(self, key, Detour)
            end
        end

        return OriginalNewIndex(self, key, value)
    end))
    RspyHooks.NewIndex = OriginalNewIndex
end

function RspyStart()
    if RemoteSpyActive then
        return
    end

    -- Validate required capabilities
    if not Capabilities.hookmetamethod then
        Log(LOG_ERROR, "Executor missing function 'hookmetamethod' - required for Remote Spy")
        return
    end
    if not Capabilities.hookfunction then
        Log(LOG_ERROR, "Executor missing function 'hookfunction' - required for Remote Spy")
        return
    end
    if not Capabilities.getnamecallmethod then
        Log(LOG_ERROR, "Executor missing function 'getnamecallmethod' - required for Remote Spy")
        return
    end
    if not Capabilities.newcclosure then
        Log(LOG_ERROR, "Executor missing function 'newcclosure' - required for Remote Spy")
        return
    end

    -- Warn about missing optional capabilities
    if not Capabilities.getcallingscript then
        Log(LOG_WARNING, "Executor missing function 'getcallingscript' - calling script information will not be shown in the Remote Spy")
    end
    if not Capabilities.getcallbackvalue then
        Log(LOG_WARNING, "Executor missing function 'getcallbackvalue' - incoming RemoteFunction calls may not be detected by the Remote Spy")
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
    for instance, callbacks in pairs(RspyDetouredCallbacks) do
        if instance and typeof(instance) == 'Instance' then
            instance.OnClientInvoke = callbacks.original
        end
    end

    -- Clear all session state
    RspyRemoteToId = {}
    RspyNextRemoteId = 1
    RspyCallIdToCallData = {}
    RspyNextCallId = 1
    RspyHooks = {}
    RspyConnections = {}
    RspyLogConnectionFunctions = {}
    RspyDetouredCallbacks = {}

    Log(LOG_SUCCESS, 'Remote spy stopped')
end

function RspyDecompile(CallId)
    local CallData = RspyCallIdToCallData[CallId]

    if not CallData then
        SendMessage('rspy_decompiled', {
            callId = CallId,
            source = '-- This remote has no associated call data'
        })
        return
    end

    local ScriptInstance = CallData.callingScript

    if not ScriptInstance then
        SendMessage('rspy_decompiled', {
            callId = CallId,
            source = '-- This remote call has no associated calling script'
        })
        return
    end

    local Source = '-- Your executor does not support script decompilation'

    if Capabilities.decompile then
        local Success, Result = pcall(decompile, ScriptInstance)
        if Success then
            Source = Result
        else
            Source = ('--[[\nError decompiling: %s\n]]--'):format(tostring(Result))
        end
    end

    SendMessage('rspy_decompiled', {
        callId = CallId,
        source = Source
    })
end

function RspyGenerateCode(CallId)
    local CallData = RspyCallIdToCallData[CallId]

    if not CallData then
        SendMessage('rspy_generated_code', {
            callId = CallId,
            code = '-- This remote has no associated call data'
        })
        return
    end

    local RemotePath = BuildInstancePath(CallData.instance)
    local Code = ''

    -- Generate code based on direction and class
    if CallData.direction == 'outgoing' then
        -- Outgoing: remote:FireServer() or remote:InvokeServer()
        local MethodName = (CallData.className == 'RemoteFunction') and 'InvokeServer' or 'FireServer'

        -- Build arguments table
        if #CallData.arguments > 0 then
            local ArgsRepr = Serialize(CallData.arguments)
            Code = 'local args = ' .. ArgsRepr .. '\n\n'
            Code = Code .. RemotePath .. ':' .. MethodName .. '(table.unpack(args, 1, #args))'
        else
            Code = RemotePath .. ':' .. MethodName .. '()'
        end
    else
        -- Incoming: OnClientEvent:Connect() or OnClientInvoke callback
        if CallData.className == 'RemoteFunction' then
            Code = RemotePath .. '.OnClientInvoke = function('

            -- Add parameters
            if #CallData.arguments > 0 then
                local ParamNames = {}
                for i = 1, #CallData.arguments do
                    table.insert(ParamNames, 'arg' .. i)
                end
                Code = Code .. table.concat(ParamNames, ', ')
            end

            Code = Code .. ')\n    -- Handle request\n'

            -- Add return if there were return values
            if CallData.returnValues and #CallData.returnValues > 0 then
                local ReturnStrs = {}
                for i = 1, #CallData.returnValues do
                    table.insert(ReturnStrs, Serialize(CallData.returnValues[i]))
                end
                Code = Code .. '    return ' .. table.concat(ReturnStrs, ', ') .. '\n'
            end

            Code = Code .. 'end'
        else
            -- RemoteEvent or UnreliableRemoteEvent
            Code = RemotePath .. '.OnClientEvent:Connect(function('

            -- Add parameters
            if #CallData.arguments > 0 then
                local ParamNames = {}
                for i = 1, #CallData.arguments do
                    table.insert(ParamNames, 'arg' .. i)
                end
                Code = Code .. table.concat(ParamNames, ', ')
            end

            Code = Code .. ')\n    -- Handle event\nend)'
        end
    end

    SendMessage('rspy_generated_code', {
        callId = CallId,
        code = Code
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
