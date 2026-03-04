local cjson = require "cjson.safe"

local AugIDBFC = {}

function AugIDBFC.new_channel(args)
  return {
    channelid  = args.channelid,
    didowner   = args.didowner,
    patternid  = args.patternid,
    mobilityscope = args.mobilityscope or "ONBODY",
    topology   = args.topology or "STAR",
    maxhops    = args.maxhops or 3,
    jittermaxms = args.jittermaxms or 80,
    offlinesnapshothash = args.offlinesnapshothash or "",
    safetytag  = args.safetytag or "",
  }
end

-- Serialize a credential request over the BFC channel.
function AugIDBFC.build_vc_request(channel, vc_kind)
  return cjson.encode{
    kind    = "AugID_VC_REQUEST",
    vc_kind = vc_kind,          -- e.g. "age_over_18"
    did     = channel.didowner,
    chan    = channel.channelid,
    pattern = channel.patternid,
  }
end

return AugIDBFC
