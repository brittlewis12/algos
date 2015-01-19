Code.require_file("inversion.ex", "lib")

case File.read("IntegerArray.txt") do
  {:ok, contents} ->
    contents |> String.split |> Enum.map(&String.to_integer/1)
             |> Inversion.count |> IO.puts
  {:error, reason} ->
    IO.puts("error: #{reason}")
end
