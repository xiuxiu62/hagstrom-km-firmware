import ctypes
from enum import Enum

hagstrom = ctypes.cdll.LoadLibrary("hagstrom.dll")
hagstrom.initialize_emulator.argtypes = [ctypes.c_char_p]
hagstrom.write_message.argtypes = [ctypes.c_char_p, ctypes.c_uint64]
# hagstrom.mouse_move.argtypes = [ctypes.c_uint16, ctypes.c_uint16, ctypes.c_uint64]
# hagstrom.mouse_click.argtypes = [ctypes.c_uint8, ctypes.c_uint64]
# hagstrom.mouse_scroll.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint64]

class ResponseCode(Enum):
    Ok = 0
    Uninitialized = 1
    DataFormatting = 2
    DeviceNotFound = 3
    LockPoisoned = 4


class KeyCode(Enum): 
    Zero = 0
    One = 1
    Two = 2
    Three = 3
    Four = 4
    Five = 5
    Six = 6
    Seven = 7
    Eight = 8
    Nine = 9

    A = 10
    B = 11
    C = 12
    D = 13
    E = 14
    F = 15
    G = 16
    H = 17
    I = 18
    J = 19
    K = 20
    L = 21
    M = 22
    N = 23
    O = 24
    P = 25
    Q = 26
    R = 27
    S = 28
    T = 29
    U = 30
    V = 31
    W = 32
    X = 33
    Y = 34
    Z = 35

    Tilde = 36
    Space = 37
    Dash = 38
    Equal = 39
    LBracket = 40
    RBracket = 41
    BackSlash = 42
    SemiColon = 43
    Quote = 44
    Comma = 45
    Period = 46
    ForwardSlash = 47
    BackSpace = 48
    Tab = 49
    Caps = 50
    Enter = 51
    Shift = 52
    Control = 53
    Alt = 54
    Super = 55
    Escape = 56
    Left = 57
    Up = 58
    Down = 59
    Right = 60
    
    F1 = 61
    F2 = 62
    F3 = 63
    F4 = 64
    F5 = 65
    F6 = 66
    F7 = 67
    F8 = 68
    F9 = 69
    F10 = 70
    F11 = 71
    F12 = 72


def initialize(serial_port: str):
    handle_response(hagstrom.initialize_emulator(serial_port.encode("utf-8")))
    
def write_message(message: str, timeout: int):
    handle_response(hagstrom.write_message(message.encode("utf-8"), timeout))
    
def write_command(keycodes: list[KeyCode], timeout: int):
    bytes = list(map(lambda keycode: keycode.value, keycodes))
    keycodes = (ctypes.c_uint8 * len(bytes))(*bytes) 
    handle_response(hagstrom.write_command(keycodes, timeout))
    
def handle_response(response: ResponseCode):
    if response != 0:
        match ResponseCode(response):
            case ResponseCode.Uninitialized: 
                print("Emulator uninitialized")
            case ResponseCode.DataFormatting:
                print("Data improperlly formatted")
            case ResponseCode.DeviceNotFound:
                print("Device not found")
            case ResponseCode.LockPoisoned:
                print("Lock poisoned")
                
        quit()
         
class Emulator:
    def __init__(serial_port):
        this.id = serial_port 
        handle_response(hagstrom.initialize_emulator(serial_port.encode("utf-8")))
    
    def write_message(message, timeout):
        handle_response(hagstrom.write_message(message.encode("utf-8"), timeout))
    
    def write_command(keycodes, timeout):
        bytes = list(map(lambda keycode: keycode.value, keycodes))
        keycodes = (ctypes.c_uint8 * len(bytes))(*bytes) 
        handle_response(hagstrom.write_command(keycodes, timeout))
        
    # def move(x: int, y: int, timeout: int):
    #     handle_response(hagstrom.mouse_move(x, y, timeout))
    
    # def click(button: MouseButton, timeout: int):
    #     handle_response(hagstrom.mouse_click(button.value, timeout))
    
    # def scroll(direction: ScrollDirection, magnitude: ScrollMagnitude, timeout: int):
    #     handle_response(hagstrom.mouse_scroll(direction.value, magnitude.value, timeout))


# class MouseButton(Enum):
#     Left = 0
#     Middle = 1
#     Right = 2
    
# class ScrollDirection(Enum):
#     Up = hex("0x80")
#     Down = hex("0x00")
    
# class ScrollMagnitude(Enum):
#     Seven = hex("0x70")
#     Six = hex("0x60")
#     Five = hex("0x50")
#     Four = hex("0x40")
#     Three = hex("0x30")
#     Two = hex("0x20")
#     One = hex("0x10")
#     Zero = hex("0x00")

# def hex(value: str) -> int:
#     return int(value, base = 16)
