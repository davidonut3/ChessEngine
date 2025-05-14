import pygame
from math import floor

BLACK = (0, 0, 0)
GREY = (125, 125, 125)
WHITE = (255, 255, 255)

def main(width):
    pygame.init()

    height = 8

    screen_width = 60 * width
    screen_height = 60 * height
    screen = pygame.display.set_mode((screen_width, screen_height), pygame.RESIZABLE)
    pygame.display.set_caption('Chess Bit Flipper')
    clock = pygame.time.Clock()

    board = [[0 for _ in range(width)] for _ in range(height)]

    running = True

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_SPACE:
                    for rank in board:
                        for square in rank:
                            print(square, end='')
                    print()
                if event.key == pygame.K_BACKSPACE:
                    board = [[0 for _ in range(width)] for _ in range(height)]
            if event.type == pygame.MOUSEBUTTONDOWN:
                mouse = pygame.mouse.get_pos()
                rank = floor(mouse[1] / 60)
                file = floor(mouse[0] / 60)
                if board[rank][file] == 0:
                    board[rank][file] = 1
                else:
                    board[rank][file] = 0

        screen.fill(WHITE)

        for rank in range(height):
            for file in range(width):
                rect = pygame.rect.Rect(file * 60, rank * 60, 60, 60)
                pygame.draw.rect(screen, WHITE if not board[rank][file] else BLACK, rect)

        for rank in range(height + 1):
            pygame.draw.line(screen, GREY, (0, 60 * rank), (screen_width, 60 * rank), 2)
        
        for file in range(width + 1):
            pygame.draw.line(screen, GREY, (60 * file, 0), (60 * file, screen_height), 2)

        pygame.display.update()
        clock.tick(60)

    pygame.quit()