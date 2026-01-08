获取余额使用`provider.get_balance()`
不过这个函数我在alloy文档里没有找到，还是通过写注释然后代码补全获得了提示。

关于单位换算wei<-->ether，我参考的官方alloy文档：https://alloy.rs/using-primitive-types/using-big-numbers#parsing-and-formatting-units
例如：10^18 wei -> 1 ether
使用`format_uints()`函数
开头需引入`use alloy::primitives::{utils::format_units, U256};`
