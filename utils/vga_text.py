def style(fg, bg): return bg << 4 | fg 

Black = 0 	 
Blue = 1 	 
Green = 2 	 
Cyan = 3 	 
Red = 4 	 
Purple = 5 	 
Brown = 6 	 
Gray = 7 	 
DarkGray = 8 	 
LightBlue = 9 	 
LightGreen = 10 	 
LightCyan = 11 	 
LightRed = 12 	 
LightPurple = 13 	 
Yellow = 14 	 
White = 15 	 



def pad_hex(nb, pad):
    s = str(hex(nb)[2:])
    return f"0x{s:0>{pad}}"


def gen_text(text:str, color:str):
    out = [pad_hex(color << 8 | ord(char),4) for char in text]
    return out
