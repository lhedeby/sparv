public interface IClientRequest
{
    object? Handle(State state, StreamWriter writer);
}
