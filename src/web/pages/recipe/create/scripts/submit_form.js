import { createToast } from "/pages/global/components/Toast.js";

async function submitForm(event) {
  event.preventDefault();

  const formData = new FormData(event.target);
  const path = event.target.action;
  const method = event.target.method;

  const amounts = formData.getAll("amount");
  const units = formData.getAll("unit");
  const ingredients = formData.getAll("ingredient");
  const directions = formData.getAll("direction");
  const tags = formData.getAll("tag");

  console.log("ingredients", ingredients);
  console.log("directions", directions);

  let servings = formData.get("servings");
  let recipe = {
    title: formData.get("title"),
    servings: servings ? parseInt(servings) : 0,
    favorite: false,
    ingredients: [],
    directions: [],
    tags: tags,
  };

  for (let i = 0; i < ingredients.length; i++) {
    console.log(parseFloat(amounts[i]));
    recipe.ingredients.push({
      name: ingredients[i],
      amount: amounts[i] ? parseFloat(amounts[i]) : 0,
      unit: units[i],
    });
  }

  for (let i = 0; i < directions.length; i++) {
    recipe.directions.push({
      step_order: i,
      details: directions[i],
    });
  }

  const body = JSON.stringify(recipe);

  let res = await fetch(path, {
    method: method,
    headers: {
      "Content-Type": "application/json",
    },
    //   redirect: "follow",
    body: body,
  });

  if (res.ok) {
    window.location = "/recipe/all";
  } else {
    let text = await res.text();
    createToast("error", text);
  }
}

export function useSubmitForm() {
  document.getElementById("recipe_form").addEventListener("submit", submitForm);
}
