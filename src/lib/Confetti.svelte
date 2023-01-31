<script lang="ts">
  // import Particles from "svelte-particles";
  import Particles from "svelte-particles";
  import type {
    Container,
    Engine,
    IOptions,
    RecursivePartial,
  } from "tsparticles-engine";
  import { loadFull } from "tsparticles";

  interface UnsafeContainer extends Container {
    [key: string]: any;
  }

  const emitterName = "explosion-effect";

  export let shouldPlay = false;
  let particles: UnsafeContainer | undefined;

  $: if (particles) {
    console.log("particles", particles);
    if (shouldPlay) {
      particles.play();
      particles.playEmitter(emitterName);
    } else {
      particles.pauseEmitter(emitterName);
      particles.pause();
    }
  }

  let particlesConfig: RecursivePartial<IOptions> = {
    fullScreen: {
      zIndex: 1,
    },
    autoPlay: false,
    emitters: {
      name: emitterName,
      position: {
        x: 50,
        y: -20,
      },
      rate: {
        quantity: 5,
        delay: 0.15,
      },
      particles: {
        move: {
          direction: "bottom",
        },
      },
    },
    particles: {
      color: {
        value: ["#1E00FF", "#FF0061", "#E1FF00", "#00FF9E"],
      },
      move: {
        decay: 0.05,
        direction: "top",
        enable: true,
        gravity: {
          enable: true,
        },
        outModes: {
          top: "none",
          default: "destroy",
        },
        speed: {
          min: 50,
          max: 100,
        },
      },
      number: {
        value: 0,
      },
      opacity: {
        value: 1,
      },
      rotate: {
        value: {
          min: 0,
          max: 360,
        },
        direction: "random",
        animation: {
          enable: true,
          speed: 30,
        },
      },
      tilt: {
        direction: "random",
        enable: true,
        value: {
          min: 0,
          max: 360,
        },
        animation: {
          enable: true,
          speed: 30,
        },
      },
      size: {
        value: 3,
        animation: {
          enable: true,
          startValue: "min",
          count: 1,
          speed: 16,
          sync: true,
        },
      },
      roll: {
        darken: {
          enable: true,
          value: 25,
        },
        enlighten: {
          enable: true,
          value: 25,
        },
        enable: true,
        speed: {
          min: 5,
          max: 15,
        },
      },
      wobble: {
        distance: 30,
        enable: true,
        speed: {
          min: -7,
          max: 7,
        },
      },
      shape: {
        type: ["circle", "square"],
        options: {},
      },
    },
    responsive: [
      {
        maxWidth: 1024,
        options: {
          particles: {
            move: {
              speed: {
                min: 33,
                max: 66,
              },
            },
          },
        },
      },
    ],
  };

  function onLoadParticles(
    event: CustomEvent<{ particles?: Container | undefined }>
  ) {
    if (event.detail.particles) {
      particles = event.detail.particles;
    }
  }

  async function initParticles(engine: Engine) {
    await loadFull(engine);
  }
</script>

<Particles
  id="particles"
  options={particlesConfig}
  particlesInit={initParticles}
  on:particlesLoaded={onLoadParticles}
/>
