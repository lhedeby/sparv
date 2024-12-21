public record class CompletionItem
{
    public required string Label {get;set;}
    public CompletionItemKind Kind {get;set;}
    public CompletionItemLabelDetails? LabelDetails {get;set;}
    public string? Detail {get;set;}
    public MarkupContent? Documentation {get;set;}
    public string? InsertText {get;set;}
    public InsertTextFormat InsertTextFormat {get;set;}
};

public enum InsertTextFormat
{
    PlainText = 1,
    Snippet = 2,
}

public enum CompletionItemKind
{
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}
