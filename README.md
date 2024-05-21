# MyRCore

a simple RISC-V OS written in Rust for my UPC OS homework (2024 Spring)

Highly inspired by [rCore](https://github.com/rcore-os/rCore)

## Feature

- 使用内存安全语言`Rust`
- 多任务时间片轮转
- 抢占式调度
- 在环形缓冲上运行多个生产者、消费者程序
- 使用`XMake`进行构建
- 使用`RustSBI`

## Usage

Install xmake, qemu, etc. on Linux, and run commands:

```bash
xmake

xmake run run
# or debug with GDB
xmake run debug
```

## Result

多个生产者、消费者程序在同一个环形缓冲上生产和消费

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

其中，`cx`(c1, c2...)代表消费者，`px`(p1, p2...)代表生产者，每个生产者生产和消费者消费的时间周期都不同；监视程序`Monitor`每隔一秒输出一次环形缓冲区状态，10秒后将系统关机。

## Code Structure

```
├── bootloader  # RustSBI-QEMU bootloader
├── os  # 系统内核
│   ├── src
│   │   ├── sync    # Rust线程安全模块
│   │   ├── syscall # 系统调用模块
│   │   ├── task    # 进程调度模块
│   │   └── trap    # 自陷模块
│   └── target
├── user # 用户态
│   ├── src
│   │   └── bin     # 用户程序
│   └── target
└── xmake # 构建系统
    └── modules
```

## Theory

### 系统调度

利用`RISC-V`的时钟中断，实现时间片轮转。具体来讲，通过SBI设置`mtimecmp`寄存器，每隔一段时间（时间片长度）触发自陷中断`trap`，触发`yield`系统调用，系统便将当前程序挂起，并选择下一个程序运行。

生产者和消费者在等待信号量时，也可以主动触发`yield`系统调用，实现让权等待。

### 系统调用

考虑到安全性，设置内核栈和用户栈，当触发自陷中断时，先将用户栈上下文保存到内核栈，然后切换到内核栈上，再调用中断处理，根据调用号，调用相应的中断服务程序，返回结果修改上下文，之后再恢复用户栈上下文，切换回用户栈，中断返回。

### 任务切换

在自陷`yield`中断服务程序中，将当前内核栈的上下文（和上面那个上下文不一样）保存到`TaskControlBlock`中，然后恢复下一个任务的内核栈上下文，从而实现切换到下一个任务。

### 多生产者和消费者

系统没有实现虚拟地址，所有程序共用地址空间，在程序地址空间外的固定内存地址`0x84000000`，放置环形缓冲，并且设置信号量控制程序对环形缓冲的互斥访问。

具体来讲，设置对环形缓冲访问的互斥锁`FoodBuffer.lock`，生产者生产到环形缓冲的互斥锁`FoodBuffer.lock_feed`，消费者从环形缓冲消费的互斥锁`FoodBuffer.lock_eat`。生产者先获取`lock_feed`互斥锁，然后再获取`lock`互斥锁，之后才能将新值写入到环形缓冲中。消费者先获取`lock_eat`互斥锁，然后再获取`lock`互斥锁，之后才能从环形缓冲中读取值。互斥锁的获取和释放通过原子指令实现原子操作。

### Rust内存安全

那可太多了
