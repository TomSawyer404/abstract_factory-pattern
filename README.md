# Abstract Factory模式

抽象工厂（Abstract Factory）模式提供了一种方式，可以将一组具有同一主题的单独的[工厂](https://zh.wikipedia.org/wiki/工厂方法)封装起来。在正常使用中，客户端程序需要创建抽象工厂的具体实现，然后使用抽象工厂作为[接口](https://zh.wikipedia.org/w/index.php?title=接口_(资讯科技)&action=edit&redlink=1)来创建这一主题的具体对象。客户端程序不需要知道（或关心）它从这些内部的工厂方法中获得对象的具体类型，因为客户端程序仅使用这些对象的通用接口。抽象工厂模式将一组对象的实现细节与他们的一般使用分离开来。

Abstract Factory 和 Factory Method 的不同点在于，Factory Method 只是一个方法，而 Abstract Factory 是一个对象，这个对象有很多的工厂方法。

# 实战

假设北方工业有一套防御系统（Shield System），它有海基版和路基版两个型号：

- 路基版：路盾3000（LD-3000）

  <img src="https://mz.eastday.com/59089433.jpg?imageslim" alt="LD3000" style="zoom: 50%;" />

- 路基版：红旗9地空导弹（HQ-9）

  ![HQ-9](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a0/HQ-9_Surface-to-air_missiles_20170919.jpg/220px-HQ-9_Surface-to-air_missiles_20170919.jpg)



- 海基版：H/PJ-11型30毫米近防炮

  <img src="https://bkimg.cdn.bcebos.com/pic/72f082025aafa40f61f641c8a664034f78f019f6?x-bce-process=image/resize,m_lfit,w_536,limit_1/format,f_jpg" alt="PJ-1130" style="zoom: 80%;" />

- 海基版：海红旗9舰空导弹（HHQ-9）

  ![HHQ-9](https://upload.wikimedia.org/wikipedia/commons/thumb/6/63/PLANS_Changchun_%28150%29%2C_Penang_Strait%2C_Penang.jpg/200px-PLANS_Changchun_%28150%29%2C_Penang_Strait%2C_Penang.jpg)

北方工业希望在运行时创建密集阵系列的产品族，我们为每个系列的产品族创建一个工厂`LandFactory`和`ShipFactory`。每个工厂都有两个方法`create_phalanx`和`create_hq`并返回对应的产品，可以将这两个方法抽象成一个接口`AbstractFactory`。这样在运行时刻我们可以选择创建需要的产品系列。

客户可以根据需要选择 路基版本 或 海基版本 来创建 `Phalanx` 或 `HQ` 的实例：

```cpp
let style = SystemType::Land;
let phalanx = match style{
    SystemType::Land = > {let land_factory = Factory::new_shield_system(style);
        land_factory.create_phalanx()
    }
    SystemType::Ship = > {
        let ship_factory = Factory::new_shield_system(style);
        ship_factory.create_phalanx()
    }
};
phalanx.fire();
```



# 参考资料

- [抽象工厂模式-Wikipedia](https://zh.wikipedia.org/zh-cn/%E6%8A%BD%E8%B1%A1%E5%B7%A5%E5%8E%82)
- [lpxxn的示例代码](https://github.com/lpxxn/rust-design-pattern/blob/master/creational/abstract_factory.rs)
- [yukihir0的示例代码](https://github.com/yukihir0/rust_design_pattern/blob/master/abstract_factory/src/main.rs)
- [抽象工厂模式-菜鸟教程](https://www.runoob.com/design-pattern/abstract-factory-pattern.html)
- [抽象工厂模式与工厂方法模式的异同](https://stackoverflow.com/questions/5739611/what-are-the-differences-between-abstract-factory-and-factory-design-patterns)

---
