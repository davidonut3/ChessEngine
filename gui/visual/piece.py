from config import *
from visual.utils import *

class Piece:
    def __init__(self, name='PAWN', type=WHITE, perspective=WHITE, bounds=[0, 0, 0, 0], pos=(0,0)):
        self.name = name
        self.type = type
        self.bounds = bounds
        self.pos = pos

        if not perspective == WHITE:
            self.pos = invert_tile(self.pos)
            self.bounds[0], self.bounds[2] = self.bounds[2], self.bounds[0]
            self.bounds[1], self.bounds[3] = self.bounds[3], self.bounds[1]
        
        self.image = pygame.image.load(f"{MAIN_LOCATION}/pieces/{self.type}_{self.name}.png").convert_alpha()
        self.image = pygame.transform.scale_by(self.image, TILE_SIZE / 60)
        self.image_width = self.image.get_width()
        self.image_height = self.image.get_height()
    
    def draw(self, screen):
        screen.blit(self.image, (self.pos[0], self.pos[1]))

    def center_image_at(self, pos):
        self.pos = (pos[0] - self.image_width / 2, pos[1] - self.image_height / 2)
        self.pos = (max(min(self.pos[0], BOARD_SIZE - self.image_height + self.bounds[2]), -self.bounds[0]), max(min(self.pos[1], BOARD_SIZE - self.image_height + self.bounds[3]), -self.bounds[1]))