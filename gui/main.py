from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual

def perft(fen_str, per_move):
    for i in range(1,7):
        perft_check(i, fen_str, per_move)

def test():
    fen = FenPy()
    bot1 = RandomEnginePy.new_game(DEFAULT)
    bot2 = DumbEnginePy.new_game(DEFAULT)
    # PlayerVsBotMatch(bot1, player_is_white=True, fen=fen, perspective=WHITE).run_match()
    # PlayerVsPlayerMatch(fen, WHITE).run_match()
    BotVsBotMatch(bot1, bot2, delay=0.5, fen=fen, is_visual=True, perspective=WHITE).run_match()

# test()

time1 = time.time()
run_matchup_py(True, 100, 20)
print(time.time() - time1)