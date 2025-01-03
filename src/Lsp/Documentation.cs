public record class Documentation
{
    // the name used to call the function
    public required string Label { get; set; }
    public required string Title { get; set; }
    // {return type} label({ params })
    public required string Detail { get; set; }
    public required string[] DocDescription { get; set; }
    public required (string Name, string Type, string Desc)[] Parameters { get; set; }
    public required (string Type, string Desc) Returns { get; set; }
    public required string[] Examples { get; set; }
    public required string InsertText { get; set; }

    private string ParamString => string.Join("\n", Parameters.Select(param => $"- `{param.Name}` *({param.Type})*: {param.Desc}"));
    private string ReturnString => $"- *({Returns.Type})*: {Returns.Desc}";

    public CompletionItem ToCompletionItem()
    {
        return new CompletionItem()
        {
            Label = Label,
            Kind = CompletionItemKind.Function,
            LabelDetails = new("Built-in function", null),
            Detail = Detail,
            Documentation = new(
                "markdown",
                $"---\n# **{Title}**\n{string.Join("\n", DocDescription)}\n---\n# **Parameters**\n{ParamString}\n---\n# **Returns**\n{ReturnString}\n---\n# **Examples**\n{string.Join("\n", Examples)}\n"),
            InsertText = InsertText,
            InsertTextFormat = InsertText.Contains('$') ? InsertTextFormat.Snippet : InsertTextFormat.PlainText
        };
    }

    public static Documentation Print() => new Documentation
    {
        Label = "print",
        Title = "Print",
        Detail = "any print(message: any)",
        DocDescription = [
            "Prints a message to the console.",
            "Returns the parameter to enable chaining."
        ],
        Parameters = [("message", "any", "The text to print to the console.")],
        Returns = ("any", "The same value that was passed as a parameter."),
        Examples = [
            "```",
            "print(\"Hello, world!\");",
            "",
            "[1,2,3]->print->print;",
            "```"
        ],
        InsertText = "print($1)"
    };

    public static Documentation ReadInput() => new Documentation
    {
        Label = "read_input",
        Title = "Read Input",
        Detail = "string read_input()",
        DocDescription = ["Reads user input from the command line"],
        Parameters = [
        ],
        Returns = ("string", "A string with the given input"),
        Examples = [
            "```",
            "var s = read_input();",
            "```"
        ],
        InsertText = "read_input()"
    };

    public static Documentation Split() => new Documentation
    {
        Label = "split",
        Title = "Split",
        Detail = "[] split(input: string, separator: string)",
        DocDescription = ["Splits the string at the given separator."],
        Parameters = [
            ("input", "string", "The string to split"),
            ("separator", "string", "The separator to split att")
        ],
        Returns = ("[]", "A list with all the parts from the split."),
        Examples = [
            "```",
            "var parts = split(\"Hello, world!\", \", \") -> print;",
            "",
            "parts[0] == \"Hello\" // true",
            "",
            "parts[1] == \"world!\" // true",
            "```"
        ],
        InsertText = "split($1, $2)"
    };

    public static Documentation ReadFile() => new Documentation
    {
        Label = "read_file",
        Title = "Read File",
        Detail = "string read_file(path: string)",
        DocDescription = ["Tries to read the file specified in the path."],
        Parameters = [
            ("path", "string", "The path to the specified file relative to the current file."),
        ],
        Returns = ("string", "A string with the contents of the file."),
        Examples = [
            "```",
            "var file_contents = read_file(\"./foo.txt\");",
            "",
            "file_contents -> print; // prints the content of the file",
            "```"
        ],
        InsertText = "read_file($1)"
    };

    public static Documentation Len() => new Documentation
    {
        Label = "len",
        Title = "Len",
        Detail = "number len(item: string | list)",
        DocDescription = ["Calculates the length of a string or list."],
        Parameters = [
            ("item", "string | list", "The item to calculate the length of."),
        ],
        Returns = ("number", "A number with the length of the item"),
        Examples = [
            "```",
            "var chars = len(\"foo\"); // 3",
            "",
            "var list_items = len([1, 2, 3]); // also 3",
            "```"
        ],
        InsertText = "len($1)"
    };

    public static Documentation Parse() => new Documentation
    {
        Label = "parse",
        Title = "Parse",
        Detail = "number parse(item: string)",
        DocDescription = ["Parses a string into a number."],
        Parameters = [
            ("item", "string", "The string to parse as a number."),
        ],
        Returns = ("number", "The number of the parsed string."),
        Examples = [
            "```",
            "var nine = parse(\"9\");",
            "",
            "9 / 3 == 3; // true",
            "```"
        ],
        InsertText = "parse($1)"
    };

    public static Documentation Typeof() => new Documentation
    {
        Label = "typeof",
        Title = "Typeof",
        Detail = "string typeof(item: any)",
        DocDescription = ["Returns the type of the parameter."],
        Parameters = [
            ("item", "any", "The item that you want to know the type of."),
        ],
        Returns = ("string", "A string with the name of the type (TODO)"),
        Examples = [
            "```",
            "typeof(\"9\"); // <string>",
            "typeof(9); // <number>",
            "typeof(true); // <bool>",
            "```"
        ],
        InsertText = "typeof($1)"
    };
}


