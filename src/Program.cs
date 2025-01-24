internal class Program
{
    private static async Task Main(string[] args)
    {
        // VSCode passes extra arguments when starting the lsp.
        // To avoid any issues we start the lsp server if at
        // least 1 arg is 'lsp'
        if (args.Contains("lsp"))
        {
            await LspServer.Run();
            return;
        }

        if (args.Contains("help"))
        {
            Console.WriteLine(HelpMessage);
            return;
        }

        if (args.Length != 1)
        {
            Console.WriteLine(ErrorMessage);
            return;
        }

        var command = args[0];

        if (!command.EndsWith(".sparv"))
        {
            Console.WriteLine(ErrorMessage);
            return;
        }

        Console.WriteLine($"Running: {command}");

        using var sr = new StreamReader(command);
        var source = sr.ReadToEnd();
        var parser = new Parser(source);
        var root = parser.Parse();
        if (parser.HasErrors)
        {
            foreach (var e in parser.Errors)
            {
                e.PrintError(parser.Source);
            }
            return;
        }

        var interpreter = new Interpreter();
        try
        {
            root.Interpret(interpreter);
        }
        catch (SparvException se)
        {
            se.PrintError(parser.Source);
        }

        Console.WriteLine("successfully ran program");
    }

    private static string ErrorMessage =>
    """
    Error: Invalid usage.

    To execute a Sparv file, use the following format:
        sparv <filename>

    Example:
        sparv my_script.sparv

    Hint: Ensure that the filename includes the correct path and the `.sparv` extension.

    For more details, run:
        sparv help
    """;

    private static string HelpMessage =>
    """
    To execute a Sparv file, use the following format:
        sparv <filename>

    Example:
        sparv my_script.sparv

    Hint: Ensure that the filename includes the correct path and the `.sparv` extension.

    For more details, run:
        sparv help
    """;
}

