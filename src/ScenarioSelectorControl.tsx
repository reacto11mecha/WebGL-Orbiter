import * as THREE from 'three/src/Three';
import * as React from "react";

import { MenuControl } from './MenuControl';
import { AU, AxisAngleQuaternion, CelestialBody } from './CelestialBody';
import { ContainerControl } from './ContainerControl';

import menuIconUrl from './images/menuIcon.png';

export interface ScenarioSelectorControlProps{
    // items: {title: string, value: string}[];
    onSelectScenario: (callback: (select_obj: CelestialBody) => boolean) => void,
    sendMessage: (msg: string) => void,
    setThrottle: (throttle: number) => void,
    resetTime: () => void,
    showEvent: () => void,
}

export class ScenarioSelectorControl extends React.Component<ScenarioSelectorControlProps, {visible: boolean}>{

    protected getSelectObj: () => CelestialBody;
    protected showEvent: () => void;

    constructor(props: ScenarioSelectorControlProps){
        super(props);
        this.state = {visible: false};
    }

    render(){
        const config = {
            buttonTop: 0,
            buttonHeight: 32,
            buttonWidth: 32,
            innerTitle: "Scenario Selector",
        };

        const valueElement = <div
            style={{
                border: "5px ridge #ffff7f",
                pointerEvents: "auto",
                position: "fixed",
                left: 0,
                right: 0,
                marginRight: "auto",
                marginLeft: "auto",
                width: "300px",
                top: 0,
                bottom: 0,
                marginTop: "auto",
                marginBottom: "auto",
                height: "70%",
                backgroundColor: "rgba(0,0,0,0.85)",
                fontSize: 15,
                textAlign: "center",
                fontFamily: "Sans-Serif",
                overflowY: "auto",
            }}
            >
            <div
                style={{
                    margin: 15,
                    padding: 15,
                    fontSize: 25,
                }}>
                Scenario selector
            </div>
            {
            ([
                {title: "Earth orbit", parent: "earth", semimajor_axis: 10000 / AU},
                {title: "Moon orbit", parent: "moon", semimajor_axis: 3000 / AU},
                {title: "Mars orbit", parent: "mars", semimajor_axis: 5000 / AU},
                {title: "Venus orbit", parent: "venus", semimajor_axis: 10000 / AU, ascending_node: Math.PI},
                {title: "Jupiter orbit", parent: "jupiter", semimajor_axis: 100000 / AU},
            ] as {title: string, parent: string, semimajor_axis: number,
                eccentricity?: number, rotation?: THREE.Quaternion, ascending_node?: number}[]
            ).map((item, idx) => <div
                key={idx}
                style={{
                    margin: "15px",
                    padding: "15px",
                    border: "1px solid #ffff00",
                }}
                onClick={((scenario) => (event: React.MouseEvent<HTMLDivElement, MouseEvent>) =>
                    this.props.onSelectScenario((select_obj) => {
                        const ascending_node = scenario.ascending_node || 0.;
                        var eccentricity = scenario.eccentricity || 0.;
                        var rotation = scenario.rotation ?? (() => {
                            var rotation = AxisAngleQuaternion(0, 0, 1, ascending_node - Math.PI / 2);
                            rotation.multiply(AxisAngleQuaternion(0, 1, 0, Math.PI));
                            return rotation;
                        })();
                        const parent = CelestialBody.findBody(scenario.parent);
                        if(!parent)
                            return false;
                        select_obj.setParent(parent);
                        select_obj.position = new THREE.Vector3(0, 1 - eccentricity, 0)
                            .multiplyScalar(scenario.semimajor_axis).applyQuaternion(rotation);
                        select_obj.quaternion = rotation.clone();
                        select_obj.quaternion.multiply(AxisAngleQuaternion(1, 0, 0, -Math.PI / 2));
                        select_obj.angularVelocity = new THREE.Vector3();
                        // setThrottle(0);
                        select_obj.setOrbitingVelocity(scenario.semimajor_axis, rotation);
                        select_obj.totalDeltaV = 0.;
                        select_obj.ignitionCount = 0;
                        // resetTime();
                        this.props.sendMessage('Scenario ' + scenario.title + ' Loaded!');
                        this.setState({visible: false});
                        return true;
                    })
                )(item)
                }
                >{item.title}</div>)}</div>;

        return <ContainerControl
            buttonTop={config.buttonHeight}
            buttonWidth={config.buttonWidth}
            buttonHeight={config.buttonHeight}
            iconUrl={menuIconUrl}
            caption='Scenarios'
            visible={this.state.visible}
            onSetVisible={(v) => this.setState({visible: v})}
            >{valueElement}</ContainerControl>;//, menuIconUrl, config);
    }

    setVisible(v: boolean){
        // super.setVisible.call(this, v);
        // if(this.visible){
        //     this.showEvent();
        // }
    }
}