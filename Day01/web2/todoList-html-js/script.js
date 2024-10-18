var input = document.querySelector("#input");
        var submitButton = document.querySelector("#submit");
        var clearButton = document.querySelector("#clear");
        var listItem = document.querySelector("#list");

        submitButton.addEventListener("click", function () {
            if (input.value) {
                var newListItem = document.createElement("li");
                newListItem.className = "flex items-center space-x-2 bg-white rounded-lg shadow-sm p-4";

                var checkbox = document.createElement("input");
                checkbox.type = "checkbox";
                checkbox.className = "form-checkbox h-5 w-5 text-blue-600";

                var label = document.createElement("label");
                label.textContent = input.value;
                label.className = "flex-grow";

                newListItem.appendChild(checkbox);
                newListItem.appendChild(label);

                checkbox.addEventListener("change", function () {
                    if (checkbox.checked) {
                        label.classList.add("line-through", "text-gray-500");
                    } else {
                        label.classList.remove("line-through", "text-gray-500");
                    }
                });

                listItem.appendChild(newListItem);
                input.value = "";
            }
        });

        clearButton.addEventListener("click", function () {
            listItem.innerHTML = "";
        });