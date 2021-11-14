trait ShieldSystemFactory {
    fn create_phalanx(&self) -> Box<dyn Phalanx>;
    fn create_hq(&self) -> Box<dyn HQ>;
}

struct LandFactory {}
impl ShieldSystemFactory for LandFactory {
    fn create_phalanx(&self) -> Box<dyn Phalanx> {
        Box::new(LandPhalanx {})
    }

    fn create_hq(&self) -> Box<dyn HQ> {
        Box::new(LandHQ {})
    }
}

struct ShipFactory {}
impl ShieldSystemFactory for ShipFactory {
    fn create_phalanx(&self) -> Box<dyn Phalanx> {
        Box::new(ShipPhalanx {})
    }

    fn create_hq(&self) -> Box<dyn HQ> {
        Box::new(ShipHQ {})
    }
}

trait Phalanx {
    fn fire(&self);
}

struct LandPhalanx {}
impl Phalanx for LandPhalanx {
    fn fire(&self) {
        println!("The target was destroyed in the barrage by LD-3000.");
    }
}

struct ShipPhalanx {}
impl Phalanx for ShipPhalanx {
    fn fire(&self) {
        println!("The target was destroyed in the barrage by PJ-1130.");
    }
}

trait HQ {
    fn fire(&self);
}

struct LandHQ {}
impl HQ for LandHQ {
    fn fire(&self) {
        println!("Missile launched by HQ-9");
    }
}

struct ShipHQ {}
impl HQ for ShipHQ {
    fn fire(&self) {
        println!("Missile launched by HHQ-9");
    }
}

enum SystemType {
    Land,
    Ship,
}

struct Factory;
impl Factory {
    fn new_shield_system(kind: SystemType) -> Box<dyn ShieldSystemFactory> {
        match kind {
            SystemType::Land => Box::new(LandFactory {}),
            SystemType::Ship => Box::new(ShipFactory {}),
        }
    }
}

fn main() {
    let land_factory = Factory::new_shield_system(SystemType::Land);
    let ld_3000 = land_factory.create_phalanx();
    let hq_9 = land_factory.create_hq();
    ld_3000.fire();
    hq_9.fire();

    let ship_factory = Factory::new_shield_system(SystemType::Ship);
    let pj_1130 = ship_factory.create_phalanx();
    let hhq_9 = ship_factory.create_hq();
    pj_1130.fire();
    hhq_9.fire();
}
