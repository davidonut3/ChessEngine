The Fen class that is created in rust supports the following functions that are accessible to python:

- new: access via Fen(), creates a fen object based on the default fen setup

- from_str: create a fen object from a fen notation

- to_string: returns the object in fen notation

- is_legal_move_lan: check whether the lan move is legal

- get_possible_moves_tile: get all possible moves in lan for the given tile

- get_all_possible_moves: get all possible moves in lan for the current player

- lan_to_fen: apply the move in lan to the fen

- game_ended: check whether the game ended in a win for white (1-0), a win for black (0-1) or a draw (½-½)

- in_check: check whether the current player is in check

- to_visual: return a 2D array of piece identifiers

There are other functions available, but the functions above are the most important/useful