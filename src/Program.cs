
if (args[0] == "lsp")
{
    var server = new LspServer();
    await server.Start();
    return;
}
Console.WriteLine($"Running: {args[0]}");


using var sr = new StreamReader(args[0]);
var source = sr.ReadToEnd();
var parser = new Parser(source);
var root = parser.Parse();
if (parser.HasErrors)
{
    foreach (var e in parser.Errors)
    {
        var lines = parser.Source.Split('\n');
        var maxLen = e.Line.ToString().Length + 2;
        Console.WriteLine($"{(e.Line - 1).ToString().PadRight(maxLen)}| ...");
        Console.WriteLine($"{"".PadLeft(maxLen)}|");
        Console.WriteLine($"{e.Line.ToString().PadRight(maxLen)}| {lines[e.Line]}");
        Console.WriteLine($"{"".PadRight(maxLen)}|{"".PadLeft(e.Start + 1)}{"".PadRight(e.End - e.Start, '^')}");
        Console.WriteLine($"{(e.Line + 1).ToString().PadRight(maxLen)}| ...");
        Console.WriteLine($">>> {e} at {e.Line}:{e.Start}");
    }
    return;
}

// var analyzer = new Analyzer(root);
// analyzer.Run();
// if (analyzer.HasErrors)
// {
//     Console.WriteLine("Found error!");
//     foreach (var e in analyzer.Errors)
//     {
//         var lines = parser.Source.Split('\n');
//         var maxLen = e.Line.ToString().Length + 2;
//         Console.WriteLine($"{(e.Line - 1).ToString().PadRight(maxLen)}| ...");
//         Console.WriteLine($"{"".PadLeft(maxLen)}|");
//         Console.WriteLine($"{e.Line.ToString().PadRight(maxLen)}| {lines[e.Line]}");
//         Console.WriteLine($"{"".PadRight(maxLen)}|{"".PadLeft(e.Start + 1)}{"".PadRight(e.End - e.Start, '^')}");
//         Console.WriteLine($"{(e.Line + 1).ToString().PadRight(maxLen)}| ...");
//         Console.WriteLine($">>> {e} at {e.Line}:{e.Start}");
//     }
//     Console.WriteLine("Found error!");
//     return;
// }


var interpreter = new Interpreter();
root.Interpret(interpreter);
// try
// {
//     var root = parser.Parse();
//     var interpreter = new Interpreter();
//     root.Interpret(interpreter);
// }
// catch (SparvException e)
// {
//     var lines = parser.Source.Split('\n');
//     var maxLen = e.Line.ToString().Length + 2;
//     Console.WriteLine($"{(e.Line - 1).ToString().PadRight(maxLen)}| ...");
//     Console.WriteLine($"{"".PadLeft(maxLen)}|");
//     Console.WriteLine($"{e.Line.ToString().PadRight(maxLen)}| {lines[e.Line]}");
//     Console.WriteLine($"{"".PadRight(maxLen)}|{"".PadLeft(e.Start + 1)}{"".PadRight(e.End - e.Start, '^')}");
//     Console.WriteLine($"{(e.Line + 1).ToString().PadRight(maxLen)}| ...");
//     Console.WriteLine($">>> {e} at {e.Line}:{e.Start}");
// }

// interpreter.PrintVars();
Console.WriteLine("successfully ran program");



