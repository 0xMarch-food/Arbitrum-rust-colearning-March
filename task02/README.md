1.获取余额使用`provider.get_balance()`<br />
不过这个函数我在alloy文档里没有找到，还是通过写注释然后代码补全获得了提示。

2.关于单位换算wei<-->ether，我参考的官方alloy文档：https://alloy.rs/using-primitive-types/using-big-numbers#parsing-and-formatting-units <br />
例如：10^18 wei -> 1 ether <br />
使用`format_uints()`函数 <br />
开头需引入`use alloy::primitives::{utils::format_units, U256};` <br />

3.代码运行结果<br />
<img width="1255" height="531" alt="屏幕截图 2026-01-08 220049" src="https://github.com/user-attachments/assets/1d40a835-88db-4efd-8be0-4b77aac86243" />
