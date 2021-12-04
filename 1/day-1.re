let test_input_1 = [1, 2, 3, 4, 5, 6, 7]; // Should compute to 4
let test_input_2 = [7, 6, 5, 4, 3, 2, 1]; // Should compute to 0
let test_input_3 = [1, 1, 1, 1, 1, 1, 1]; // Should compute to 0

let calc_sliding_window = (input: list(int)) => switch(input) {
	| [e1, e2, e3, e4] => (e1 + e2 + e3) < (e2 + e3 + e4)
	| _ => false
} ? 1 : 0;

// Compares l with r. If l < r, add one to initial_count. Returns the inital count
let rec depth = (input: list(int), initial_count: int) => switch(input) {
	| [l, r, ...rest] => get_decentions([r] @ rest, initial_count + (l < r ? 1 : 0))
	| _ => initial_count
};

// Compares (1, 2, 3) with (2, 3, 4). If l < r, add one to initial_count. Returns the inital count
let rec depth2 = (input: list(int), initial_count: int) => switch(input) {
	| [e1, e2, e3, e4, ...rest] => get_decentions_triple([e2, e3, e4] @ rest, (initial_count + calc_sliding_window([e1, e2, e3, e4])))
	| _ => initial_count
};

// Application entry
let () = Sys.argv |> Array.to_list
	// Filter out cmd name
	|> r => switch(r) {
		| [_] => []
		| [_, ...rest] => rest
		| _ => []
	}
	// Map inner from string to int
	|> List.map(r => Int32.of_string(r) |> Int32.to_int)
	// Get the actual value
	|> r => depth2(r, 0)
	// Cast and print
	|> Int.to_string |> r => "Total raises " ++ r |> print_endline
