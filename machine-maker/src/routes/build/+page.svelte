<script lang="ts">
  import type { Point } from "../../../../state-view/bindings/Point";
  import { onMount } from "svelte";
  // svg.addEventListener('mousedown', startDrag);
  // svg.addEventListener('mousemove', drag);
  // svg.addEventListener('mouseup', endDrag);
  // svg.addEventListener('mouseleave', endDrag);

  let selectedElement: SVGCircleElement | undefined = undefined;
  let offset: Point = { x: 0, y: 0 };
  let svg: SVGSVGElement;
  let states: Array<Point> = [];
  let scaleFactor = 5.0;
  let svgWidth = 0;
  let svgHeight = 0;
  window.addEventListener("resize", resize);

  function resize() {
    svgWidth = svg.clientWidth;
    svgHeight = svg.clientHeight;
  }
  onMount(resize);

  function startDrag(
    evt: MouseEvent & {
      currentTarget: EventTarget & SVGSVGElement;
    },
  ) {
    if (
      evt.target instanceof SVGCircleElement &&
      evt.target.classList.contains("draggable")
    ) {
      selectedElement = evt.target;
      offset = getMousePosition(evt);
      offset.x -= parseFloat(
        (selectedElement.getAttributeNS(null, "cx") as string).toString(),
      );

      offset.y -= parseFloat(
        (selectedElement.getAttributeNS(null, "cy") as string).toString(),
      );
      console.log("start " + selectedElement);
    }
  }
  function drag(
    evt: MouseEvent & {
      currentTarget: EventTarget & SVGSVGElement;
    },
  ) {
    if (selectedElement == undefined) {
      return;
    }
    evt.preventDefault();
    let coord = getMousePosition(evt);
    selectedElement.setAttributeNS(null, "cx", (coord.x - offset.x).toString());
    selectedElement.setAttributeNS(null, "cy", (coord.y - offset.y).toString());
    console.log(states);
  }
  function endDrag(evt: MouseEvent) {
    selectedElement = undefined;
  }

  function getMousePosition(evt: MouseEvent): Point {
    var CTM = svg.getScreenCTM();
    if (CTM == null) {
      return {
        x: -1,
        y: -1,
      };
    }
    return {
      x: (evt.clientX - CTM.e) / CTM.a,
      y: (evt.clientY - CTM.f) / CTM.d,
    };
  }

  function addState() {
    states = [
      ...states,
      { x: Math.random() * 20 + 5, y: Math.random() * 10 + 5 },
    ];
    console.log(states);
  }

  function zoomIn() {
    scaleFactor *= 0.8;
  }

  function zoomOut() {
    scaleFactor *= 1.2;
  }
</script>

<h1>Hi from builds</h1>

<button on:click={addState}>Add State</button>
<button on:click={zoomIn}>zoom in</button>
<button on:click={zoomOut}>zoom out</button>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<svg
  xmlns="http://www.w3.org/2000/svg"
  on:mousedown={startDrag}
  on:mousemove={drag}
  on:mouseup={endDrag}
  on:mouseleave={endDrag}
  bind:this={svg}
  viewBox="0 0 {svgWidth * scaleFactor} {svgHeight * scaleFactor}"
  class="big"
>
  <rect
    x="0"
    y="0"
    width={svgWidth * scaleFactor}
    height={svgHeight * scaleFactor}
    fill="#fafafa"
  />
  {#each states as { x, y }}
    <circle class="draggable" cx={x} cy={y} r="3"></circle>
  {/each}
</svg>

<style>
  /* .static { */
  /*   cursor: not-allowed; */
  /* } */
  .draggable {
    cursor: move;
  }
</style>
