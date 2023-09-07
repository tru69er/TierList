<script lang="ts">
  const tiers = [
    { name: "Jod", color: "Red", albums: [] },
    { name: "A", color: "Orange", albums: [] },
    { name: "B", color: "Yellow", albums: [] },
    { name: "C", color: "Green", albums: [] },
    { name: "Dogshit", color: "Blue", albums: [] },
  ];

  const albums = [
    {
      pos: [0, 0],
      path: "./src/lib/images/amnesiac.png",
    },
    {
      pos: [0, 1],
      path: "./src/lib/images/angles.jpg",
    },
    {
      pos: [0, 2],
      path: "./src/lib/images/bends.png",
    },
    {
      pos: [0, 3],
      path: "./src/lib/images/comedownmachine.jpg",
    },
    {
      pos: [0, 4],
      path: "./src/lib/images/firstimpressionsofearth.jpg",
    },
    {
      pos: [0, 5],
      path: "./src/lib/images/hailtothethief.png",
    },
    {
      pos: [0, 6],
      path: "./src/lib/images/inrainbows.png",
    },
    {
      pos: [1, 0],
      path: "./src/lib/images/isthisit.png",
    },
    {
      pos: [1, 1],
      path: "./src/lib/images/kida.png",
    },
    {
      pos: [1, 2],
      path: "./src/lib/images/kingoflimbs.png",
    },
    {
      pos: [1, 3],
      path: "./src/lib/images/moonshapedpool.png",
    },
    {
      pos: [1, 4],
      path: "./src/lib/images/newabnormal.jpg",
    },
    {
      pos: [1, 5],
      path: "./src/lib/images/okcomputer.png",
    },
    {
      pos: [1, 6],
      path: "./src/lib/images/pablohoney.png",
    },
    {
      pos: [2, 0],
      path: "./src/lib/images/roomonfire.png",
    },
  ];

  let selState = 0;

  let selIndex = [0, 0];

  function move(e: KeyboardEvent) {
    switch (e.key) {
      case "h":
        if (selIndex[1] > 0) {
          selIndex[1]--;
        }
        break;
      case "j":
        if (selIndex[0] < 2) {
          selIndex[0]++;
        }
        if (selIndex[0] === 2) {
          selIndex[1] = 0;
        }
        break;
      case "k":
        if (selIndex[0] > 0) {
          selIndex[0]--;
        }
        break;
      case "l":
        if (selIndex[0] !== 2 && selIndex[1] < 6) {
          selIndex[1]++;
        }
        break;
      default:
        break;
    }
  }

  function match(index: number[], _: number[]): boolean {
    return index[0] === selIndex[0] && index[1] === selIndex[1];
  }
</script>

<div class="page">
  <div class="container">
    <div class="title">Radiohead and Strokes Albums Tier List</div>
    <div class="tiers">
      {#each tiers as tier (tier)}
        <div class="row">
          <div class="tier {tier.color} ">
            <span class="tiername">{tier.name}</span>
          </div>
          <div class="albums" />
        </div>
      {/each}
    </div>
  </div>
  <div class="albums {selState === 0 ? 'pickAlbum' : ''}">
    {#each albums as album (album)}
      <img
        src={album.path}
        alt=""
        class={match(album.pos, selIndex) ? "selAlbum" : ""}
      />
    {/each}
  </div>
</div>

<svelte:window on:keydown={move} />

<style>
  .page {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    width: 100%;
  }

  .container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    align-items: center;
    width: 70%;
  }

  .title {
    font-size: xx-large;
    color: white;
    padding: 2% 0;
  }

  .tiers {
    width: 100%;
  }

  .row {
    display: grid;
    grid-template-columns: 10% 90%;
    background-color: rgb(41, 40, 40);

    border-top: 1px solid black;
    border-bottom: 1px solid black;
  }

  .tier {
    padding: 30%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .tiername {
    font-size: x-large;
  }

  .albums {
    margin: 2.5% 0;
    display: grid;
    grid-template-columns: repeat(7, 14.7%);
    width: 68%;
    padding: 1%;
  }

  img {
    width: 80%;
    height: 80%;
  }

  .pickAlbum {
    border: 1px solid green;
  }

  .selAlbum {
    border: 4px solid white;
  }

  .Red {
    background-color: rgb(245, 80, 80);
  }

  .Orange {
    background-color: rgb(255, 154, 71);
  }

  .Yellow {
    background-color: rgb(236, 236, 85);
  }

  .Green {
    background-color: greenyellow;
  }

  .Blue {
    background-color: cyan;
  }
</style>
