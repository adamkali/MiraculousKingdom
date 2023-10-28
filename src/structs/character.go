package structs

type Character struct {
    ID string `json:"id"`
    GameID string `json:"game_id"`
    Name string `json:"name"`
    Description string `json:"description"`
    Class string `json:"class"`
    MilitaryMight Might `json:"military_might"`
    CultureMight Might `json:"economic_might"`
    ScienceMight Might `json:"science_might"`
    DiplomacyMight Might `json:"diplomacy_might"`
    ReligionMight Might `json:"religion_might"`
    EspionageMight Might `json:"espionage_might"`
    Abilities []Ability `json:"abilities"`
}

type Might struct {
    Level int `json:"level"`
    Experience int `json:"experience"`
    Tokens int `json:"tokens"`
}

type Ability struct {
    ID string `json:"id"`
    Name string `json:"name"`
    Description string `json:"description"`
    MightLevel int `json:"might_level"`
    MightType string `json:"might_type"`
}

type CharacterRequest struct {
    GameID string `json:"game_id"`
    Name string `json:"name"`
    Description string `json:"description"`
    Class string `json:"class"`
    MilitaryMight int `json:"military_might"`
    CultureMight int `json:"economic_might"`
    ScienceMight int `json:"science_might"`
    DiplomacyMight int `json:"diplomacy_might"`
    ReligionMight int `json:"religion_might"`
    EspionageMight int `json:"espionage_might"`
}
