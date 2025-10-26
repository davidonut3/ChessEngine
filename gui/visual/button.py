from config import *
from visual.utils import *

class Button:
    def __init__(self, topleft, width, height, font, text, color=Color.DTAKE, active_color=Color.ACTIVE):
        self.rect = pygame.rect.Rect(topleft[0], topleft[1], width, height)
        self.color = color
        self.active_color = active_color
        self.text = text
        self.clicked = False

        self.text_surface = font.render(self.text, True, Color.BLACK)
        self.default_color = self.color

    def update(self, mouse_pos, mouse_down):
        if not self.rect.collidepoint(mouse_pos):
            self.clicked = False
            self.color = self.default_color
            return False
        
        self.color = self.active_color

        if mouse_down and not self.clicked:
            self.clicked = True
            self.color = self.default_color
            return True
        
        if not mouse_down:
            self.clicked = False
    
    def draw(self, screen):
        pygame.draw.rect(screen, self.color, self.rect)
        screen.blit(self.text_surface, (self.rect.left + 5, self.rect.top + 5))
        