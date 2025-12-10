package server

import (
	"time"
)

type IsAdultToken struct {
	TokenID                 []byte
	HolderPseudonymousDID   string
	JurisdictionCode        string
	AgeThreshold            uint8
	AgeThresholdSatisfied   bool
	IssuerDID               string
	MethodClass             string
	IssuedAt                time.Time
	NotBefore               time.Time
	NotAfter                time.Time
	BindingHash             []byte
	Signature               []byte
}

func VerifyIsAdultToken(tok IsAdultToken, siteOrigin string, now time.Time) (bool, string) {
	if now.Before(tok.NotBefore) || now.After(tok.NotAfter) {
		return false, "expired"
	}
	if !tok.AgeThresholdSatisfied {
		return false, "threshold_not_met"
	}
	if !verifyIssuerSignature(tok) {
		return false, "bad_signature"
	}
	if !verifyBindingHash(tok, siteOrigin) {
		return false, "origin_binding_failed"
	}
	logNonIdentifying(tok)
	return true, "ok"
}
