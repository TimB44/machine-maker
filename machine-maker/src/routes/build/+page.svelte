<script lang="ts">
  import type { Point } from "../../../../state-view/bindings/Point";
  // svg.addEventListener('mousedown', startDrag);
  // svg.addEventListener('mousemove', drag);
  // svg.addEventListener('mouseup', endDrag);
  // svg.addEventListener('mouseleave', endDrag);

  let selectedElement: SVGCircleElement | undefined = undefined;
  let svg: SVGSVGElement;
  let states: Array<Point> = [];

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
    selectedElement.setAttributeNS(null, "cx", coord.x.toString());
    selectedElement.setAttributeNS(null, "cy", coord.y.toString());
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
</script>

<h1>Hi from builds</h1>

<button on:click={addState}>Add State</button>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 30 20"
  on:mousedown={(e) => startDrag(e)}
  on:mousemove={drag}
  on:mouseup={endDrag}
  on:mouseleave={endDrag}
  bind:this={svg}
>
  <rect x="0" y="0" width="30" height="20" fill="#fafafa" />
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
