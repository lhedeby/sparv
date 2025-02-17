public static class Documentation
{
    private record class NativeFunctionDoc
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
    }

    public static List<CompletionItem> CompletionItems => _nativeFunctions.Select(doc => doc.ToCompletionItem()).ToList();
    public static CompletionItem CompletionItem(string identifier) => _nativeFunctions.First(doc => doc.Label == identifier).ToCompletionItem();
    public static bool IsNative(string identifier) => _nativeFunctions.Exists(x => x.Label == identifier);

    private static readonly List<NativeFunctionDoc> _nativeFunctions = new List<NativeFunctionDoc>
    {
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
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
        },
        new NativeFunctionDoc
        {
            Label = "abs",
            Title = "Abs",
            Detail = "number abs(param: number)",
            DocDescription = ["Return the absolute of the param."],
            Parameters = [
                ("param", "number", "The item that you want to know the type of."),
            ],
            Returns = ("number", "The absolute value of the number."),
            Examples = [
                "```",
                "var this_is_true = abs(-9) == 9;",
                "var this_is_true = abs(9) == 9;",

                "var this_is_not_true = abs(-9) == -9;",
                "var this_is_not_true = abs(9) == -9;",
                "```"
            ],
            InsertText = "abs($1)"
        },
        new NativeFunctionDoc
        {
            Label = "time",
            Title = "Time",
            Detail = "number time()",
            DocDescription = ["Return the the miliseconds elapsed since the program started."],
            Parameters = [],
            Returns = ("number", "The amount of miliseconds."),
            Examples = [
                "```",
                "var start = time();",
                "for i in 0:100000 { print(\"hello\"); }",

                "var end = time();",
                "print(\"elapsed time: \" + (end-start) + \" ms\");",
                "```"
            ],
            InsertText = "time()"
        },
    };
}


