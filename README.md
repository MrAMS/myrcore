# my rCore playground

a simple OS written in Rust for my OS homework

## Feature:
- time-sharing multitasking
- preemptive scheduling
- multiple producer consumer apps
- use XMake as build tool

## Usage

install xmake, qemu, etc.

```bash
xmake

xmake run run
# or debug
xmake run debug
```

## Output

```
[rustsbi] RustSBI version 0.3.1, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000f02
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!
Monitor start
c1 start
c2 start
p1 start
p2 start
[Monitor] Buf feed at 0, eat at 0
p1 wakeup at 1007
p1 feed 1000
c2 wakeup at 1007
c2 eat 1000
c2 wakeup at 2007
p1 wakeup at 2007
p1 feed 1001
p2 wakeup at 2007
[Monitor] Buf feed at 2, eat at 1
c2 eat 1001
p2 feed 2000
c2 wakeup at 3007
c2 eat 2000
p1 wakeup at 3007
p1 feed 1002
[Monitor] Buf feed at 4, eat at 3
[Monitor] Buf feed at 4, eat at 3
c2 wakeup at 4007p1 wakeup at 4007
p1 feed 1003
p2 wakeup at 4007
p2 feed 2001

c2 eat 1002
p1 wakeup at 5007
p1 feed 1004
[Monitor] Buf feed at 7, eat at 4
c1 wakeup at 5007
c1 eat 1003
c2 wakeup at 5007
c2 eat 2001
c2 wakeup at 6007
c2 eat 1004
p1 wakeup at 6007
p1 feed 1005
p2 wakeup at 6007
p2 feed 2002
[Monitor] Buf feed at 9, eat at 7
[Monitor] Buf feed at 9, eat at 7
c2 wakeup at 7007p1 wakeup at 7007
p1 feed 1006

c2 eat 1005
[Monitor] Buf feed at 10, eat at 8
c2 wakeup at 8007
c2 eat 2002
p1 wakeup at 8007
p1 feed 1007
p2 wakeup at 8007
p2 feed 2003
[Monitor] Buf feed at 12, eat at 9
[Monitor] Time to exit
[kernel] Halt
```