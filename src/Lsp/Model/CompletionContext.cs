public record class CompletionContext(CompletionTriggerKind TriggerKind, string? TriggerCharacter);

public enum CompletionTriggerKind 
{
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}
