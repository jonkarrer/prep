import { StagedIngredient } from "./StagedIngredient.js";
import { StagedDirection } from "./StagedDirection.js";
import { IngredientController } from "./IngredientController.js";
import { DirectionController } from "./DirectionController.js";
import { TagController } from "./TagController.js";
import { StaticIngredient } from "./StaticIngredient.js";
import { StaticDirection } from "./StaticDirection.js";
import { StaticTag } from "./StaticTag.js";
import { StagedTag } from "./StagedTag.js";

customElements.define("staged-ingredient", StagedIngredient);
customElements.define("staged-direction", StagedDirection);
customElements.define("staged-tag", StagedTag);
customElements.define("ingredient-controller", IngredientController);
customElements.define("direction-controller", DirectionController);
customElements.define("tag-controller", TagController);
customElements.define("static-ingredient", StaticIngredient);
customElements.define("static-direction", StaticDirection);
customElements.define("static-tag", StaticTag);
