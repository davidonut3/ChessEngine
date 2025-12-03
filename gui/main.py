from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual

def perft(fen_str, per_move):
    for i in range(1,7):
        perft_check(i, fen_str, per_move)

def test():
    fen = FenPy()
    botOther = SortedEnginePy.new_game(DEFAULT)

    bot0 = DumbEnginePy.new_game(DEFAULT)
    bot1 = RandomEnginePy.new_game(DEFAULT)
    bot2 = SimpleEnginePy.new_game(DEFAULT)
    bot3 = AlphaEnginePy.new_game(DEFAULT)
    bot4 = SortedEnginePy.new_game(DEFAULT)
    PlayerVsBotMatch(bot4, player_is_white=True, fen=fen, perspective=WHITE).run_match()
    # PlayerVsPlayerMatch(fen, WHITE).run_match()
    # BotVsBotMatch(bot4, botOther, delay_seconds=0, fen=fen, is_visual=True, perspective=WHITE).run_match()

# test()

run_matchup_py(True, 100, 500)

# rust_access()