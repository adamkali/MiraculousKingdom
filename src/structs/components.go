package structs

type ButtonData struct {
	Endpoint string
	Target   string
	Action   string
	Text     string
	SvgName  string
	Method   string
}

type SubmitButtonData struct {
	SvgName string
	Text    string
}

type LinkData struct {
	Href    string
	Text    string
	SvgName string
}

type InputData struct {
	Label    string
	Disabled bool
	Value    string
	Name     string
}
