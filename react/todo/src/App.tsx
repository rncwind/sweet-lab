import "./App.css"
import { useState, useRef } from 'react';
import { v4 as uuid } from 'uuid';


type Todo = {
  uuid?: String,
  body: String,
  completeState: boolean,
}

function App() {
  let [todosList, setTodosList] = useState<Todo[]>([
    { body: "Test Todo", completeState: false },
    { body: "Test Todo 2", completeState: true },
  ]);

  function handleComplete(i: number) {
    let nextTodos = todosList.slice();
    nextTodos[i].completeState = !(nextTodos[i].completeState);
    setTodosList(nextTodos);
  };

  function deleteTodo(target: Todo) {
    // Is this _really_ the best way to handle it? The other way i saw was
    // splice, but that also seems kinda grody.
    let nextTodos = todosList.slice();
    nextTodos = nextTodos.filter((td) => td !== target);
    setTodosList(nextTodos);
  }

  function handleSubmit(e) {
    // Boilerplate to get the form data.
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);
    let containsData = Array.from(formData.keys()).includes("todoText");
    const formJson = Object.fromEntries(formData.entries());
    if (formJson.todoContent === null || containsData === false) {
      alert("Todo doesn't have a valid field! Not adding it");
      return;
    }

    // Gen a UUID
    const todoUUID = uuid();
    let newTodo: Todo = { uuid: todoUUID, body: formJson.todoText.toString(), completeState: false };
    let serialized = JSON.stringify(newTodo);
    console.log("Serializing with Key: " + todoUUID);
    console.log("Serailized value is: " + serialized);

    // Clone our todos list
    let nextTodos = todosList.slice();
    // Append a new todo to our list.
    nextTodos.push(newTodo);
    // Set our todoslist to be our new list.
    setTodosList(nextTodos);
    // Update localstorage
    localStorage.setItem(todoUUID, serialized);
  }


  return (
    <>
      <section id="todoContainer">
        {renderTodos(todosList, handleComplete, deleteTodo)}
      </section>
      <section id="addTodoContainer">
        <AddTodoForm handleSubmit={handleSubmit} />
      </section>
    </>
  );
}

function renderTodos(todos: Array<Todo>, onClick, deleteTodo) {
  return todos.map((entry: Todo, idx: number) => {
    return <TodoEntry key={idx} todo={entry} onClick={() => onClick(idx)} onDeleteClick={() => deleteTodo(entry)} />
  });
}

function TodoEntry({ todo, onClick, onDeleteClick }) {
  let label = '';
  let classname = '';
  if (todo.completeState === true) {
    label = "☒";
    classname = "completeStateComplete";
  } else {
    label = "☐"
    classname = "completeStateIncomplete"
  }
  return (
    <div className="todoEntry">
      <button onClick={onClick} className={classname}>{label}</button>
      <p>{todo.body}</p>
      <button onClick={onDeleteClick}>Delete</button>
    </div>
  )
}

function AddTodoForm({ handleSubmit }) {
  return (
    <>
      <form method="post" onSubmit={handleSubmit} id="submitNewTodoForm">
        <div id="spacer"></div>
        <textarea id="addTodoTextArea" name="todoText" rows={1} />
        <button type="submit">+</button>
      </form>
    </>
  )
}

export default App
