-- ============================================================================
-- FILE: augid_bfc_client.lua
-- NAMESPACE: AugID.BFC.Client
-- VERSION: 1.0.0
-- DESCRIPTION: Lightweight Lua client for Augmented-ID BFC channel operations
-- COMPATIBILITY: Lua 5.3+, LuaJIT 2.1+
-- DEPENDENCIES: cjson.safe, crypto (for signing)
-- ============================================================================

local cjson = require "cjson.safe"
local crypto = require "crypto"

-- Module table
local AugIDBFC = {}
AugIDBFC._VERSION = "1.0.0"

-- ============================================================================
-- CONSTANTS
-- ============================================================================

AugIDBFC.TOKEN_VALIDITY_SECS = 300  -- 5 minutes
AugIDBFC.REQUIRED_NEURORIGHTS = {
    "no_exclusion_basic_services",
    "no_score_from_inner_state",
    "revocable_at_will",
}

AugIDBFC.CONSENT_STATES = {
    CONFIRMED = "CONFIRMED",
    DENY = "DENY",
    SUSPENDED = "SUSPENDED",
}

AugIDBFC.INTERFACE_TYPES = {
    IMPLANTED_NFC = "implanted_nfc",
    ECO_NFC = "EcoNFC",
    XR_RIG = "xr_rig",
    MOBILE_APP = "mobile_app",
}

-- ============================================================================
-- CHANNEL CREATION
-- ============================================================================

--- Create a new Bounded Forward Channel
-- @param args Table with channel parameters
-- @return Channel object or nil on error
function AugIDBFC.new_channel(args)
    if not args.channelid then
        return nil, "channelid required"
    end
    if not args.didowner then
        return nil, "didowner required"
    end
    if not args.patternid then
        return nil, "patternid required"
    end
    
    local channel = {
        channelid = args.channelid,
        didowner = args.didowner,
        patternid = args.patternid,
        mobilityscope = args.mobilityscope or "ONBODY",
        topology = args.topology or "STAR",
        maxhops = args.maxhops or 3,
        jittermaxms = args.jittermaxms or 80,
        offlinesnapshothash = args.offlinesnapshothash or "",
        safetytag = args.safetytag or "",
        roh_limit = args.roh_limit or 0.3,
        neurorights_envelope = args.neurorights_envelope or AugIDBFC.REQUIRED_NEURORIGHTS,
        created_at = os.time(),
        status = "Active",
    }
    
    return channel
end

-- ============================================================================
-- TOKEN GENERATION
-- ============================================================================

--- Generate a BfcToken.v1 from citizen data
-- @param citizen Table with citizen identity data
-- @param consent_state String (CONFIRMED, DENY, SUSPENDED)
-- @param interface_type String (implanted_nfc, EcoNFC, xr_rig, mobile_app)
-- @return Token table or nil on error
function AugIDBFC.generate_token(citizen, consent_state, interface_type)
    if not citizen or not citizen.did then
        return nil, "citizen.did required"
    end
    
    local now = os.time()
    local valid_until = now + AugIDBFC.TOKEN_VALIDITY_SECS
    
    local token = {
        tokenid = AugIDBFC.generate_uuid(),
        token_version = "v1",
        generated_at = now,
        valid_until = valid_until,
        walletdid = citizen.did,
        interface_type = interface_type or AugIDBFC.INTERFACE_TYPES.MOBILE_APP,
        aiconsentstate = consent_state or AugIDBFC.CONSENT_STATES.DENY,
        caps_ok = {
            spend_cap_ok = citizen.spend_cap_ok or false,
            prompt_cap_ok = citizen.prompt_cap_ok or false,
            id_check_ok = citizen.id_check_ok or false,
        },
        eco_flags = {
            EcoImpactScore_band = citizen.eco_band or "Participant",
            Eaccessibility = citizen.eaccessibility or false,
            ServiceClassBasic = "Enabled",
        },
        neurorights_flags = citizen.neurightsflags or AugIDBFC.REQUIRED_NEURORIGHTS,
        snapshot_hash = citizen.organichainroot or "",
    }
    
    return token
end

--- Sign a token with device key
-- @param token Table (token data)
-- @param private_key String (hex-encoded private key)
-- @return Signed token or nil on error
function AugIDBFC.sign_token(token, private_key)
    local token_json = cjson.encode(token)
    if not token_json then
        return nil, "failed to serialize token"
    end
    
    -- Hash the token
    local hash = crypto.hash("sha256", token_json)
    
    -- Sign the hash (placeholder - actual implementation would use Ed25519)
    local signature = crypto.sign("ed25519", hash, private_key)
    
    token.issuer_signature = signature
    return token
end

-- ============================================================================
-- TOKEN VALIDATION
-- ============================================================================

--- Validate a BfcToken.v1
-- @param token Table (token data)
-- @return true if valid, or nil + error message
function AugIDBFC.validate_token(token)
    if not token then
        return nil, "token is nil"
    end
    
    -- Check version
    if token.token_version ~= "v1" then
        return nil, "invalid token version: " .. tostring(token.token_version)
    end
    
    -- Check expiry
    local now = os.time()
    if now > token.valid_until then
        return nil, "token expired"
    end
    
    -- Check neurorights flags
    for _, required_flag in ipairs(AugIDBFC.REQUIRED_NEURORIGHTS) do
        local found = false
        for _, flag in ipairs(token.neurorights_flags or {}) do
            if flag == required_flag then
                found = true
                break
            end
        end
        if not found then
            return nil, "missing neurorights flag: " .. required_flag
        end
    end
    
    -- Check DID format
    if not token.walletdid or not token.walletdid:match("^bostrom1[a-z0-9]+$") then
        return nil, "invalid DID format"
    end
    
    -- Check snapshot hash format
    if token.snapshot_hash and not token.snapshot_hash:match("^sha256:[a-f0-9]+$") then
        return nil, "invalid snapshot hash format"
    end
    
    return true
end

--- Check if consent state allows an operation
-- @param token Table (token data)
-- @param operation String (operation type)
-- @return true if allowed, false otherwise
function AugIDBFC.consent_allows_operation(token, operation)
    if token.aiconsentstate == AugIDBFC.CONSENT_STATES.CONFIRMED then
        return true
    elseif token.aiconsentstate == AugIDBFC.CONSENT_STATES.SUSPENDED then
        -- Only basic services and emergency operations allowed
        return operation == "BasicService" or operation == "Emergency"
    else
        return false
    end
end

-- ============================================================================
-- MESSAGE BUILDING
-- ============================================================================

--- Build a VC request over BFC channel
-- @param channel Table (BFC channel)
-- @param vc_kind String (e.g., "age_over_18", "health_fitness")
-- @return JSON-encoded message or nil on error
function AugIDBFC.build_vc_request(channel, vc_kind)
    if not channel or not channel.channelid then
        return nil, "invalid channel"
    end
    
    local message = {
        kind = "AugID_VC_REQUEST",
        vc_kind = vc_kind,
        did = channel.didowner,
        chan = channel.channelid,
        pattern = channel.patternid,
        timestamp = os.time(),
    }
    
    return cjson.encode(message)
end

--- Build a payment authorization message
-- @param channel Table (BFC channel)
-- @param amount Number (transaction amount)
-- @param recipient String (recipient DID)
-- @return JSON-encoded message or nil on error
function AugIDBFC.build_payment_auth(channel, amount, recipient)
    if not channel or not channel.channelid then
        return nil, "invalid channel"
    end
    
    local message = {
        kind = "AugID_PAYMENT_AUTH",
        amount = amount,
        recipient = recipient,
        did = channel.didowner,
        chan = channel.channelid,
        pattern = channel.patternid,
        timestamp = os.time(),
    }
    
    return cjson.encode(message)
end

--- Build an identity verification message
-- @param channel Table (BFC channel)
-- @param token Table (BfcToken.v1)
-- @return JSON-encoded message or nil on error
function AugIDBFC.build_identity_verify(channel, token)
    if not channel or not channel.channelid then
        return nil, "invalid channel"
    end
    
    local message = {
        kind = "AugID_IDENTITY_VERIFY",
        token = token,
        did = channel.didowner,
        chan = channel.channelid,
        pattern = channel.patternid,
        timestamp = os.time(),
    }
    
    return cjson.encode(message)
end

-- ============================================================================
-- UTILITY FUNCTIONS
-- ============================================================================

--- Generate a UUID v4
-- @return UUID string
function AugIDBFC.generate_uuid()
    local template = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'
    return string.gsub(template, '[xy]', function(c)
        local v = (c == 'x') and math.random(0, 15) or math.random(8, 11)
        return string.format('%x', v)
    end)
end

--- Encode data to base64
-- @param data String
-- @return Base64-encoded string
function AugIDBFC.base64_encode(data)
    local b = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
    return ((data:gsub('.', function(x)
        local r, b = '', x:byte()
        for i = 8, 1, -1 do
            r = r .. (b % 2 ^ i - b % 2 ^ (i - 1) > 0 and '1' or '0')
        end
        return r;
    end) .. '0000'):gsub('%d%d%d?%d?%d?%d?', function(x)
        if (#x < 6) then return '' end
        local c = 0
        for i = 1, 6 do
            c = c + (x:sub(i, i) == '1' and 2 ^ (6 - i) or 0)
        end
        return b:sub(c + 1, c + 1)
    end) .. ({'', '==', '='})[#data % 3 + 1])
end

--- Decode base64 data
-- @param data String (base64-encoded)
-- @return Decoded string
function AugIDBFC.base64_decode(data)
    local b = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
    data = string.gsub(data, '[^' .. b .. '=]', '')
    return (data:gsub('.', function(x)
        if (x == '=') then return '' end
        local r, f = '', (b:find(x) - 1)
        for i = 6, 1, -1 do
            r = r .. (f % 2 ^ i - f % 2 ^ (i - 1) > 0 and '1' or '0')
        end
        return r;
    end):gsub('%d%d%d?%d?%d?%d?%d?%d?', function(x)
        if (#x ~= 8) then return '' end
        local c = 0
        for i = 1, 8 do
            c = c + (x:sub(i, i) == '1' and 2 ^ (8 - i) or 0)
        end
        return string.char(c)
    end))
end

-- ============================================================================
-- CHANNEL STATE MANAGEMENT
-- ============================================================================

--- Serialize channel state for offline storage
-- @param channel Table (BFC channel)
-- @return JSON-encoded channel state
function AugIDBFC.serialize_channel(channel)
    return cjson.encode(channel)
end

--- Deserialize channel state from offline storage
-- @param json_string String (JSON-encoded channel)
-- @return Channel table or nil on error
function AugIDBFC.deserialize_channel(json_string)
    local channel = cjson.decode(json_string)
    if not channel then
        return nil, "failed to decode channel"
    end
    return channel
end

--- Check if channel is expired
-- @param channel Table (BFC channel)
-- @param max_age_secs Number (maximum channel age in seconds)
-- @return true if expired, false otherwise
function AugIDBFC.is_channel_expired(channel, max_age_secs)
    max_age_secs = max_age_secs or 86400  -- Default 24 hours
    local now = os.time()
    return (now - channel.created_at) > max_age_secs
end

-- ============================================================================
-- ERROR HANDLING
-- ============================================================================

AugIDBFC.errors = {
    INVALID_CHANNEL = "INVALID_CHANNEL",
    TOKEN_EXPIRED = "TOKEN_EXPIRED",
    CONSENT_DENIED = "CONSENT_DENIED",
    SIGNATURE_INVALID = "SIGNATURE_INVALID",
    NEURORIGHTS_VIOLATION = "NEURORIGHTS_VIOLATION",
    OFFLINE_SNAPSHOT_EXPIRED = "OFFLINE_SNAPSHOT_EXPIRED",
}

--- Create an error object
-- @param error_type String (from AugIDBFC.errors)
-- @param message String (error details)
-- @return Error table
function AugIDBFC.create_error(error_type, message)
    return {
        error_type = error_type,
        message = message,
        timestamp = os.time(),
    }
end

-- ============================================================================
-- MODULE EXPORT
-- ============================================================================

return AugIDBFC
