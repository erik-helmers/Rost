def virt_to_indexes(addr):
    a = hex(addr) 
    

def split_virt_addr(addr):
    offset = addr & 0o7777
    p1 = (addr >> 12) & 0o777
    p2 = (addr >> 21) & 0o777
    p3 = (addr >> 30) & 0o777
    p4 = (addr >> 39) & 0o777
    return (p4, p3, p2, p1, offset)

def valid_virt_addr(addr):
    p4, _,_,_,_= split_virt_addr(addr)
    if not (p4 & 1 << 9): return addr >> 48 == 0
    else: return addr >> 48 == 0xffff