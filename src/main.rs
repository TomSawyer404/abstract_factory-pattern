trait ShieldSystemFactory {
    fn create_phalanx(&self) -> Box<dyn Phalanx>;
    fn create_hq(&self) -> Box<dyn HQ>;
}

trait Phalanx {
    fn fire(&self);
}

trait HQ {
    fn fire(&self);
}

enum SystemType {
    Land,
    Ship,
}

struct PhalanxEntity {}
impl Phalanx for PhalanxEntity {
    fn fire(&self) {
        println!("The target was destroyed in the barrage.");
    }
}

struct HQEntity {}
impl HQ for HQEntity {
    fn fire(&self) {
        println!("Missile launched!");
    }
}

struct LandFactory {}
impl ShieldSystemFactory for LandFactory {
    fn create_phalanx(&self) -> Box<dyn Phalanx> {
        Box::new(PhalanxEntity {})
    }

    fn create_hq(&self) -> Box<dyn HQ> {
        Box::new(HQEntity {})
    }
}

struct ShipFactory {}
impl ShieldSystemFactory for ShipFactory {
    fn create_phalanx(&self) -> Box<dyn Phalanx> {
        Box::new(PhalanxEntity {})
    }

    fn create_hq(&self) -> Box<dyn HQ> {
        Box::new(HQEntity {})
    }
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
