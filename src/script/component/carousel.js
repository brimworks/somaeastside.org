/**

Usage:

<web-carousel interval="1000">
  <a href="/post/worship"><img src="/static/img/worship.webp" alt="Worship" /></a>
  <a href="/post/womens-retreat"><img src="/static/img/womens-retreat.webp" alt="Women's Retreat" /></a>
</web-carousel>

Template:
 
 data-carousel-item: defines how each list item should be displayed.
 data-carousel-content: defines where each web-carousel child will be placed within the item.
 data-carousel-jump: onClick, jumps to this item, adds 
 data-carousel-prev: onClick, goes to next item
 data-carousel-next: onClick, goes to prev item
 */
const TEMPLATE = document.createElement("template");
TEMPLATE.innerHTML =
`

<div class="relative h-56 md:h-96  aspect-video">
  <button data-carousel-prev class="absolute top-0 start-0 z-30 flex items-center justify-center h-full px-4 cursor-pointer group focus:outline-none">
    <span class="inline-flex items-center justify-center w-10 h-10 rounded-full bg-white/30 dark:bg-gray-800/30 group-hover:bg-white/50 dark:group-hover:bg-gray-800/60 group-focus:ring-4 group-focus:ring-white dark:group-focus:ring-gray-800/70 group-focus:outline-none">
      <svg class="w-4 h-4 text-white dark:text-gray-800 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
        <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 1 1 5l4 4"/>
      </svg>
      <span class="sr-only">Previous</span>
    </span>
  </button>
  <ul class="relative h-56 md:h-96 aspect-video overflow-hidden rounded-lg bg-black">
    <li data-carousel-item class="absolute duration-700 top-1/2 left-1/2 ease-in-out w-full flex items-center">
      <div data-carousel-content class="h-full -translate-x-1/2 -translate-y-1/2" />
    </li>
  </ul>
  <div class="absolute z-30 flex -translate-x-1/2 space-x-3 rtl:space-x-reverse bottom-5 left-1/2">
    <button data-carousel-jump class="w-3 h-3 rounded-full hover:ring-2 hover:ring-white/70 dark:hover:ring-gray-800/70 bg-white/30 dark:bg-gray-800/90"></button>
  </div>
  <button data-carousel-next class="absolute top-0 end-0 z-30 flex items-center justify-center h-full px-4 cursor-pointer group focus:outline-none">
    <span class="inline-flex items-center justify-center w-10 h-10 rounded-full bg-white/30 dark:bg-gray-800/30 group-hover:bg-white/50 dark:group-hover:bg-gray-800/60 group-focus:ring-4 group-focus:ring-white dark:group-focus:ring-gray-800/70 group-focus:outline-none">
      <svg class="w-4 h-4 text-white dark:text-gray-800 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
        <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="m1 9 4-4-4-4"/>
      </svg>
      <span class="sr-only">Next</span>
    </span>
  </button>
</div>
`;

function removeChildren(node) {
  while (node.hasChildNodes()) {
    node.removeChild(node.firstChild);
  }
}
function nextRotatedSibling(node) {
  if (null == node.nextElementSibling) {
    return node.parentNode.firstElementChild;
  }
  return node.nextElementSibling;
}
function previousRotatedSibling(node) {
  if (null == node.previousElementSibling) {
    return node.parentNode.lastElementChild;
  }
  return node.previousElementSibling;
}
function indexOf(node) {
  return Array.prototype.indexOf.call(node.parentNode.children, node);
}
class Carousel extends HTMLElement {
  static observedAttributes = ["interval"];
  #intervalDelay;
  #interval;
  constructor() {
    super();
    this.#intervalDelay = 5000;
    this.#interval = setInterval(this.rotate.bind(this), this.#intervalDelay);
  }
  connectedCallback() {
    const template = TEMPLATE.content.cloneNode(true);
    const templateItem = template.querySelector("[data-carousel-item]");
    const templateItemList = templateItem.parentNode;    
    removeChildren(templateItemList);
    const templateJump = template.querySelector("[data-carousel-jump]");
    const templateJumpList = templateJump.parentNode;
    removeChildren(templateJumpList);
    var idx = 0;
    const halfIdx = this.children.length / 2;
    for (const child of this.children) {
      const itemContent = templateItem.cloneNode(true);
      itemContent.querySelector("[data-carousel-content]").replaceChildren(...child.cloneNode(true).children);
      this.styleSlide(itemContent, 0 == idx ? "center" : (halfIdx < idx ? "left" : "right"));
      templateItemList.append(itemContent);
      const jump = templateJump.cloneNode(true);
      jump.addEventListener("click", this.handleJump.bind(this, idx));
      const altTextElm = child.querySelector("[alt]");
      if (altTextElm && altTextElm.getAttribute("alt")) {
        jump.setAttribute("aria-label", altTextElm.getAttribute("alt"));
      }
      this.styleJump(jump, 0 == idx);
      templateJumpList.append(jump);
      idx += 1;
    }
    template.querySelector("[data-carousel-prev]").addEventListener("click", this.handlePrev.bind(this));
    template.querySelector("[data-carousel-next]").addEventListener("click", this.handleNext.bind(this));
    this.replaceChildren(template);
  }
  attributeChangedCallback(name, oldValue, newValue) {
    const reset = this.#intervalDelay != newValue;
    this.#intervalDelay = newValue;
    if (reset) {
      this.resetInterval();
    }
  }
  resetInterval() {
    clearInterval(this.#interval);
    this.#interval = setInterval(this.rotate.bind(this), this.#intervalDelay);
  }
  handleJump(idx, e) {
    this.resetInterval();
    let items = this.querySelectorAll("[data-carousel-item]");
    let activeIdx = indexOf(this.querySelector("[data-carousel-item='active']"));
    var rotateRight;
    if (Math.abs(idx - activeIdx) >= items.length / 2) {
      rotateRight = activeIdx > idx;
    } else {
      rotateRight = idx > activeIdx;
    }
    if (rotateRight) {
      this.slideTo(items[activeIdx], items[idx], nextRotatedSibling(items[idx]));
    } else {
      this.slideTo(previousRotatedSibling(items[idx]), items[idx], items[activeIdx]);
    }
  }
  handleNext(e) {
    this.resetInterval();
    this.rotate();
  }
  handlePrev(e) {
    this.resetInterval();
    let right = this.querySelector("[data-carousel-item='active']");
    let middle = previousRotatedSibling(right)
    this.slideTo(previousRotatedSibling(middle), middle, right);
  }
  rotate() {
    let left = this.querySelector("[data-carousel-item='active']");
    let middle = nextRotatedSibling(left);
    this.slideTo(left, middle, nextRotatedSibling(middle));
  }
  styleJump(jump, isActive) {
    jump.classList.remove(
      "bg-white/80",
      "bg-white/30",
    );
    if (isActive) {
      jump.classList.add("bg-white/80");
      jump.setAttribute("aria-current", "true");
    } else {
      jump.classList.add("bg-white/30");
      jump.removeAttribute("aria-current");
    }
  }
  styleSlide(slide, position) {
    slide.classList.remove(
      '-translate-x-full',
      'translate-x-full',
      'translate-x-0',
      'z-20',
      'z-10',
      'invisible',
    );
    var isActive = false;
    switch (position) {
    case "left":
      slide.classList.add('-translate-x-full', 'z-10', 'invisible');
      break;
    case "right":
      slide.classList.add('translate-x-full', 'z-10', 'invisible')
      break;
    case "center":
      slide.classList.add('translate-x-0', 'z-20');
      isActive = true;
      break;
    }
    if (isActive) {
      slide.setAttribute("aria-current", "true");
      slide.setAttribute("data-carousel-item", "active")
    } else {
      slide.removeAttribute("aria-current");
      slide.setAttribute("data-carousel-item", "")
    }
  }
  slideTo(left, center, right) {
    this.styleSlide(left, "left");
    this.styleSlide(right, "right");
    // Always set center fields last in case center == right or left.
    this.styleSlide(center, "center");

    // Now style the jumps:
    let centerIdx = indexOf(center);
    var idx = 0;
    for (const jump of this.querySelectorAll("[data-carousel-jump]")) {
      this.styleJump(jump, idx == centerIdx);
      idx += 1;
    }
  }
}
customElements.define('web-carousel', Carousel);