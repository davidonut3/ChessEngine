from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual

def test():
    game = DEFAULT

    fen = FenPy()
    fen = fen.from_str(game)
    botOther = AlphaEnginePy.new_game(game)

    bot0 = DumbEnginePy.new_game(game)
    bot1 = RandomEnginePy.new_game(game)
    bot2 = SimpleEnginePy.new_game(game)
    bot3 = AlphaEnginePy.new_game(game)
    bot4 = SortedEnginePy.new_game(game)
    PlayerVsBotMatch(bot4, 1000, player_is_white=True, fen=fen, perspective=WHITE).run_match()
    # PlayerVsPlayerMatch(fen, WHITE).run_match()
    # BotVsBotMatch(bot4, botOther, delay_seconds=0, fen=fen, is_visual=True, perspective=WHITE).run_match()

# run_matchup_py(True, 100, 500)

# rust_access()

validate_move_gen_py()