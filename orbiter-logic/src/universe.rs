use crate::{
    celestial_body::{AddPlanetParams, CelestialBody, OrbitalElements},
    dyn_iter::{Chained, DynIterMut, MutRef},
    session::SessionId,
    GMsun, Quaternion, Vector3, AU,
};
use cgmath::{Rad, Rotation3, Zero};
use rand::prelude::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Debug)]
pub struct Universe {
    pub bodies: Vec<CelestialBody>,
    pub root: usize,
    pub id_gen: usize,
    sim_time: f64,
    start_time: f64,
    time: usize,
    pub time_scale: f64,
}

impl Universe {
    pub fn new() -> Self {
        let mut this = Self {
            bodies: vec![],
            root: 0,
            id_gen: 0,
            sim_time: 0.,
            start_time: 0.,
            time: 0,
            time_scale: 1.,
        };

        let sun = CelestialBody::new(
            &mut this,
            None,
            Vector3::zero(),
            "#ffffff".to_string(),
            GMsun,
            "sun".to_string(),
            OrbitalElements::default(),
        );
        let sun_id = sun.id;
        this.add_body(sun);

        let rad_per_deg = std::f64::consts::PI / 180.;

        let params = AddPlanetParams {
            axial_tilt: 23.4392811 * rad_per_deg,
            rotation_period: ((23. * 60. + 56.) * 60. + 4.10),
            // soi: 5e5,
            quaternion: Quaternion::new(1., 0., 0., 0.),
            angular_velocity: Vector3::zero(),
        };

        let earth = CelestialBody::from_orbital_elements(
            &mut this,
            Some(sun_id),
            OrbitalElements {
                semimajor_axis: 1.,
                eccentricity: 0.0167086,
                inclination: 0.,
                ascending_node: -11.26064 * rad_per_deg,
                argument_of_perihelion: 114.20783 * rad_per_deg,
                epoch: 0.,
                mean_anomaly: 0.,
                soi: 1.,
            },
            params,
            398600. / AU / AU / AU,
            6534.,
            "earth".to_string(),
        );
        let earth_id = earth.id;

        this.add_body(earth);

        let mut rocket = CelestialBody::from_orbital_elements(
            &mut this,
            Some(earth_id),
            OrbitalElements {
                semimajor_axis: 10000. / AU,
                eccentricity: 0.,
                inclination: 0.,
                ascending_node: 0.,
                argument_of_perihelion: 0.,
                epoch: 0.,
                mean_anomaly: 0.,
                soi: 1.,
            },
            AddPlanetParams::default(),
            100. / AU / AU / AU,
            0.1,
            "rocket".to_string(),
        );

        let rot = <Quaternion as Rotation3>::from_angle_x(Rad(std::f64::consts::PI / 2.))
            * <Quaternion as Rotation3>::from_angle_y(Rad(std::f64::consts::PI / 2.));
        rocket.quaternion = rot;

        this.add_body(rocket);

        let moon = CelestialBody::from_orbital_elements(
            &mut this,
            Some(earth_id),
            OrbitalElements {
                semimajor_axis: 384399. / AU,
                eccentricity: 0.048775,
                inclination: -11.26064 * rad_per_deg,
                ascending_node: 100.492 * rad_per_deg,
                argument_of_perihelion: 114.20783 * rad_per_deg, //275.066 * rad_per_deg,
                epoch: 0.,
                mean_anomaly: 0.,
                soi: 1e5,
            },
            AddPlanetParams::default(),
            4904.8695 / AU / AU / AU,
            1737.1,
            "moon".to_string(),
        );

        this.add_body(moon);

        let mars = CelestialBody::from_orbital_elements(
            &mut this,
            Some(sun_id),
            OrbitalElements {
                semimajor_axis: 1.523679,
                eccentricity: 0.0935,
                inclination: 1.850 * rad_per_deg,
                ascending_node: 49.562 * rad_per_deg,
                argument_of_perihelion: 286.537 * rad_per_deg,
                epoch: 0.,
                mean_anomaly: 0.,
                soi: 3e5,
            },
            AddPlanetParams::default(),
            42828. / AU / AU / AU,
            3389.5,
            "mars".to_string(),
        );

        this.add_body(mars);

        let jupiter = CelestialBody::from_orbital_elements(
            &mut this,
            Some(sun_id),
            OrbitalElements {
                semimajor_axis: 5.204267,
                eccentricity: 0.048775,
                inclination: 1.305 * rad_per_deg,
                ascending_node: 100.492 * rad_per_deg,
                argument_of_perihelion: 275.066 * rad_per_deg,
                epoch: 0.,
                mean_anomaly: 0.,
                soi: 10e6,
            },
            AddPlanetParams::default(),
            126686534. / AU / AU / AU,
            69911.,
            "jupiter".to_string(),
        );

        this.add_body(jupiter);

        this
    }

    pub fn new_rocket(&mut self) -> SessionId {
        let earth_id = self
            .bodies
            .iter()
            .find(|body| body.name == "earth")
            .map(|body| body.id);

        let rad_per_deg = std::f64::consts::PI / 180.;

        let mut rng = thread_rng();

        let mut rocket = CelestialBody::from_orbital_elements(
            self,
            earth_id,
            OrbitalElements {
                semimajor_axis: rng.gen_range(10000.0..20000.) / AU,
                eccentricity: rng.gen_range(0.0..0.5),
                inclination: rng.gen_range(0.0..30. * rad_per_deg),
                ascending_node: rng.gen_range(0.0..360. * rad_per_deg),
                argument_of_perihelion: rng.gen_range(0.0..360. * rad_per_deg),
                epoch: 0.,
                mean_anomaly: 0.,
                soi: 1.,
            },
            AddPlanetParams {
                axial_tilt: 0.,
                rotation_period: 0.,
                quaternion: Quaternion::new(1., 0., 0., 0.),
                angular_velocity: Vector3::zero(),
            },
            100. / AU / AU / AU,
            0.1,
            format!("rocket{}", self.id_gen),
        );

        let rot = <Quaternion as Rotation3>::from_angle_x(Rad(std::f64::consts::PI / 2.))
            * <Quaternion as Rotation3>::from_angle_y(Rad(std::f64::consts::PI / 2.));
        rocket.quaternion = rot;

        let session_id = SessionId::new();
        rocket.session_id = Some(session_id);

        self.add_body(rocket);

        session_id
    }

    fn add_body(&mut self, body: CelestialBody) {
        let body_id = body.id;
        if let Some(parent) = body.parent {
            let parent = &mut self.bodies[parent];
            // println!("Add {} to {}", ret.lock().unwrap().name, parent.name);
            parent.children.push(body_id);
        }
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        fn split_bodies(
            bodies: &'_ mut [CelestialBody],
            i: usize,
        ) -> (
            &mut CelestialBody,
            impl DynIterMut<Item = CelestialBody> + '_,
        ) {
            let (first, mid) = bodies.split_at_mut(i);
            let (center, last) = mid.split_first_mut().unwrap();
            (center, Chained(MutRef(first), MutRef(last)))
        }

        let mut bodies = std::mem::take(&mut self.bodies);

        let div = 100;
        for _ in 0..div {
            for i in 0..bodies.len() {
                let (center, chained) = split_bodies(&mut bodies, i);
                center.simulate_body(chained, self.time_scale, div as f64);
            }
        }
        for i in 0..bodies.len() {
            let (center, chained) = split_bodies(&mut bodies, i);
            center.update(chained);
        }
        self.bodies = bodies;
        self.time += 1;
        self.sim_time += self.time_scale;
    }

    pub fn get_time(&self) -> usize {
        self.time
    }

    pub fn get_sim_time(&self) -> f64 {
        self.sim_time
    }
}

impl Serialize for Universe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("simTime", &self.sim_time)?;
        map.serialize_entry("startTime", &self.start_time)?;
        map.serialize_entry("bodies", &self.bodies)?;
        map.serialize_entry("timeScale", &self.time_scale)?;
        map.end()
    }
}

pub fn serialize(this: &Universe) -> serde_json::Result<String> {
    serde_json::to_string(&this)
}

#[test]
fn test_universe() {
    let _ = Universe::new();
}
