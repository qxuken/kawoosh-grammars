-module(sample).
-export([area/1, largest/1, main/0]).

-define(LIMIT, 10).
-record(point, {x = 0.0, y = 0.0}).

%% A sample.
-spec area(tuple() | atom()) -> float().
area({circle, R}) ->
    math:pi() * R * R;
area({rect, W, H}) when W > 0 ->
    W * H;
area(_) ->
    0.0.

largest([]) ->
    none;
largest([H | T]) ->
    lists:foldl(fun(X, Acc) -> max(X, Acc) end, H, T).

length_of(#point{x = X, y = Y}) ->
    math:sqrt(X * X + Y * Y).

main() ->
    Shapes = [{circle, 1.0}, {rect, 2.0, 3.0}, empty],
    Areas = [area(S) || S <- Shapes, area(S) < ?LIMIT],
    P = #point{x = 3.0, y = 4.0},
    case largest(Areas) of
        none ->
            ok;
        A ->
            io:format("~p ~p ~s~n", [A, length_of(P), "done"])
    end,
    try list_to_integer("42") of
        N -> N + 1
    catch
        error:badarg -> 0
    end.
