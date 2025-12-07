from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual

def perft(fen_str, per_move):
    for i in range(1,7):
        perft_check(i, fen_str, per_move)

def test():
    game = "r2q1rk1/1p1bbp2/p3pn1p/3p2p1/3P3B/2PB4/PPQN1PPP/R3K2R w KQ - 0 13"

    fen = FenPy()
    fen = fen.from_str(game)
    botOther = AlphaEnginePy.new_game(game)

    bot0 = DumbEnginePy.new_game(game)
    bot1 = RandomEnginePy.new_game(game)
    bot2 = SimpleEnginePy.new_game(game)
    bot3 = AlphaEnginePy.new_game(game)
    bot4 = SortedEnginePy.new_game(game)
    PlayerVsBotMatch(bot4, 1000, player_is_white=False, fen=fen, perspective=WHITE).run_match()
    # PlayerVsPlayerMatch(fen, WHITE).run_match()
    # BotVsBotMatch(bot4, botOther, delay_seconds=0, fen=fen, is_visual=True, perspective=WHITE).run_match()

# test()

# run_matchup_py(True, 100, 500)

# rust_access()