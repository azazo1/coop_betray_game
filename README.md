# 合作背叛游戏实验

受视频 [为什么一国涨了关税，另一国必须对等报复？【差评君】](https://www.bilibili.com/video/BV1rR57zzEoC) 启发, 做此实验.

实验结果:

=== 最终排名 (100 局 400 轮) ===

| 排名 | 策略           | 所有对局总得分 | 每次对局得分 |
| ---- | -------------- | -------------- | ------------ |
| 1.   | Grudger        | 3042068        | 30420.68     |
| 2.   | Davis          | 3037791        | 30377.91     |
| 3.   | Shubik         | 3012672        | 30126.72     |
| 4.   | Grofman        | 2993874        | 29938.74     |
| 5.   | TidemanChieruzzi | 2879759        | 28797.59     |
| 6.   | TitForTat      | 2852704        | 28527.04     |
| 7.   | SteinRapoport  | 2849694        | 28496.94     |
| 8.   | Nydegger       | 2788454        | 27884.54     |
| 9.   | Tullock        | 2303232        | 23032.32     |
| 10.  | Graaskamp      | 2266925        | 22669.25     |
| 11.  | Downing        | 2018132        | 20181.32     |
| 12.  | Feld           | 1944989        | 19449.89     |
| 13.  | Random         | 1863925        | 18639.25     |
| 14.  | Joss           | 1848571        | 18485.71     |
| 15.  | Anonymous      | 1794329        | 17943.29     |

局数和轮数经多次调整都是 Grudger 处于第一名.

和视频中的 TitForTat 第一并不一致, 望感兴趣的大佬帮忙解释一下.

## 使用方法

在 Release 中下载可执行文件执行(也可以使用 `cargo build` 自行编译):

```
进行策略对局模拟
Usage: game [OPTIONS]

Options:
  -s, --simulations <SIMULATIONS>                      两个博弈策略对局的总次数 [default: 100]
  -r, --rounds-per-simulation <ROUNDS_PER_SIMULATION>  每一次对局中的回合数 [default: 400]
  -h, --help                                           Print help
  -V, --version                                        Print version
```

例子:
- `game -s 100 -r 400`: 进行模拟, 每个策略和其他每策略进行 100 次对决, 每次对决进行 400 回合.

执行完之后在当前目录生成 `match_results.csv` 和 `ranking.csv`.
