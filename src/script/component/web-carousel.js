/**

Usage:

<web-carousel interval="1000">
  <a href="/post/worship"><img src="/static/img/worship.webp" alt="Worship" /></a>
  <a href="/post/womens-retreat"><img src="/static/img/womens-retreat.webp" alt="Women's Retreat" /></a>
</web-carousel>

Minimal Expected Template:

<template id='web-carousel'>
  <div data-carousel-item><slot></slot></div>
  <div data-carousel-jump></div>
</template>

The `data-carousel-item` is cloned for each item to display in the carousel, and the
<slot></slot> is populated with each child HTML element of the <web-carousel>.

The `data-carousel-jump` is cloned for each item to display in the carousel, and this
attribute is populated with the carousel index. This is an optional part of the
template.

Note that any previous/next/jump buttons will need to specify onclick handlers that call
jump() or jumpRelative() on this component.
 */

function removeChildren(node) {
  while (node.hasChildNodes()) {
    node.removeChild(node.firstChild);
  }
}
function indexOf(node) {
  return Array.prototype.indexOf.call(node.parentNode.children, node);
}
function addDataClass(node, dataClass) {
  function addClasses(node) {
    const classes = node.getAttribute("data-class-" + dataClass);
    if (!classes) return;
    for (const className of classes.split(" ")) {
      node.classList.add(className);
    }
  }
  addClasses(node);
  for (const elm of node.querySelectorAll("[data-class-" + dataClass + "]")) {
    addClasses(elm);
  }
}
function removeDataClass(node, dataClass) {
  function removeClasses(node) {
    const classes = node.getAttribute("data-class-" + dataClass);
    if (!classes) return;
    for (const className of classes.split(" ")) {
      node.classList.remove(className);
    }
  }
  removeClasses(node);
  for (const elm of node.querySelectorAll("[data-class-" + dataClass + "]")) {
    removeClasses(elm);
  }
}
class Carousel extends HTMLElement {
  #intervalDelay;
  #interval;

  static observedAttributes = ["interval"];

  constructor() {
    super();
    this.#intervalDelay = 5000;
  }
  connectedCallback() {
    var template = document.querySelector("template#web-carousel");
    if (!template) {
      console.log("Expected <template id='web-carousel'> to be defined in the document");
      return;
    }
    template = template.content.cloneNode(true);
    const templateItem = template.querySelector("[data-carousel-item]");
    if (!templateItem) {
      console.log("Expected <template id='web-carousel'> to be define an element with the data-carousel-item attribute");
      return;
    }
    const templateItemList = templateItem.parentNode;
    removeChildren(templateItemList);
    const templateJump = template.querySelector("[data-carousel-jump]");
    const templateJumpList = templateJump ? templateJump.parentNode : null;
    if (templateJump) removeChildren(templateJumpList);
    var idx = 0;
    const halfIdx = this.children.length / 2;
    for (const child of this.children) {
      const position = 0 == idx ? "center" : (halfIdx < idx ? "left" : "right");

      const itemContent = templateItem.cloneNode(true);
      itemContent.querySelector("slot").replaceChildren(child.cloneNode(true));
      this.#stylePosition(itemContent, position);
      templateItemList.append(itemContent);
      if (templateJumpList) {
        const jump = templateJump.cloneNode(true);
        jump.setAttribute("data-carousel-jump", idx);
        const altTextElm = child.querySelector("[alt]");
        if (altTextElm && altTextElm.getAttribute("alt")) {
          jump.setAttribute("aria-label", altTextElm.getAttribute("alt"));
        }
        this.#stylePosition(jump, position);
        templateJumpList.append(jump);
      }
      idx += 1;
    }
    this.replaceChildren(template);
    this.play();
  }
  attributeChangedCallback(name, oldValue, newValue) {
    const reset = this.#intervalDelay != newValue;
    this.#intervalDelay = newValue;
    if (reset) {
      this.#resetInterval();
    }
  }
  pause() {
    if (this.#interval) {
      clearInterval(this.#interval);
      this.#interval = null;
    }
  }
  play() {
    if (!this.#interval) {
      this.#interval = setInterval(this.#moveToCenterRelative.bind(this, 1), this.#intervalDelay);
    }
  }
  jump(idx) {
    this.#resetInterval();
    this.#moveToCenter(idx);
  }
  // Specify negative offset to go backwards:
  jumpRelative(offset) {
    this.#resetInterval();
    this.#moveToCenterRelative(offset);
  }
  #resetInterval() {
    if (this.#interval) clearInterval(this.#interval);
    this.#interval = setInterval(this.#moveToCenterRelative.bind(this, 1), this.#intervalDelay);
  }
  #moveToCenterRelative(offset) {
    let currentIdx = indexOf(this.querySelector("[data-carousel-position='center']"));
    this.#moveToCenter(currentIdx + offset);
  }
  #moveToCenter(idx) {
    let items = this.querySelectorAll("[data-carousel-item]");
    idx = Math.abs((items.length + idx) % items.length);
    let newCentersPosition = items[idx].getAttribute("data-carousel-position");
    if ("center" == newCentersPosition) {
      // Already the center!
      return;
    }
    let oldCentersIdx = indexOf(this.querySelector("[data-carousel-position='center']"));
    let jumps = this.querySelectorAll("[data-carousel-jump]");
    for (let i = 1; i < (items.length + 1) / 2; i++) {
      let rightIdx = (idx + i) % items.length;
      let leftIdx = (items.length + idx - i) % items.length;
      let positionRight = "right";
      let positionLeft = "left";
      if (rightIdx == oldCentersIdx && newCentersPosition == "right") {
        positionRight = "left";
      } else if (leftIdx == oldCentersIdx && newCentersPosition == "left") {
        positionLeft = "right";
      }
      this.#stylePosition(items[rightIdx], positionRight);
      this.#stylePosition(items[leftIdx], positionLeft);
      if (jumps) {
        this.#stylePosition(jumps[rightIdx], positionRight);
        this.#stylePosition(jumps[leftIdx], positionLeft);
      }
    }
    this.#stylePosition(items[idx], "center");
    if (jumps) {
      this.#stylePosition(jumps[idx], "center");
    }
  }
  #stylePosition(elm, position) {
    if (elm.getAttribute("data-carousel-position") == position) {
      return;
    }
    removeDataClass(elm, "left");
    removeDataClass(elm, "right");
    removeDataClass(elm, "center");
    addDataClass(elm, position);
    switch (position) {
    case "left":
      elm.setAttribute("aria-current", "");
      elm.setAttribute("data-carousel-position", "left")
      break;
    case "right":
      elm.setAttribute("aria-current", "");
      elm.setAttribute("data-carousel-position", "right")
      break;
    case "center":
      elm.setAttribute("aria-current", "true");
      elm.setAttribute("data-carousel-position", "center")
      break;
    }
  }
}
customElements.define('web-carousel', Carousel);