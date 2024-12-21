public record class FormattingOptions(
    int TabSize,
    bool insertSpaces,
    bool? trimTrailingWhitespace,
    bool? insertFinalNewline,
    bool? trimFinalNewlines
// [key: string]: boolean | integer | string;
);
