import React, { useState, useEffect } from 'react';
import { to_do_list_backend } from "../../declarations/to_do_list_backend";

function App() {
  const [tasks, setTasks] = useState([]);
  const [newTask, setNewTask] = useState("");
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState("all"); // all, important, completed

  // Fetch tasks based on current filter
  const fetchTasks = async () => {
    try {
      let result;
      switch (filter) {
        case "important":
          result = await to_do_list_backend.get_important_tasks();
          break;
        case "completed":
          result = await to_do_list_backend.get_completed_tasks();
          break;
        default:
          result = await to_do_list_backend.get_tasks();
      }
      setTasks(result);
    } catch (error) {
      console.error("Error fetching tasks:", error);
    } finally {
      setLoading(false);
    }
  };

  // Initial load
  useEffect(() => {
    fetchTasks();
  }, [filter]);

  // Add new task
  const handleAddTask = async (e) => {
    e.preventDefault();
    if (!newTask.trim()) return;

    try {
      await to_do_list_backend.add_task(newTask, [], false);
      setNewTask("");
      fetchTasks();
    } catch (error) {
      console.error("Error adding task:", error);
    }
  };

  // Toggle task completion
  const handleToggleComplete = async (taskId) => {
    try {
      await to_do_list_backend.toggle_task_completion(taskId);
      fetchTasks();
    } catch (error) {
      console.error("Error toggling task completion:", error);
    }
  };

  // Toggle task importance
  const handleToggleImportant = async (taskId) => {
    try {
      await to_do_list_backend.toggle_task_importance(taskId);
      fetchTasks();
    } catch (error) {
      console.error("Error toggling task importance:", error);
    }
  };

  // Delete task
  const handleDeleteTask = async (taskId) => {
    try {
      await to_do_list_backend.delete_task(taskId);
      fetchTasks();
    } catch (error) {
      console.error("Error deleting task:", error);
    }
  };

  if (loading) {
    return <div style={styles.loading}>Loading...</div>;
  }

  return (
    <div style={styles.container}>
      <h1 style={styles.title}>To-Do List</h1>

      {/* Add Task Form */}
      <form onSubmit={handleAddTask} style={styles.form}>
        <input
          type="text"
          value={newTask}
          onChange={(e) => setNewTask(e.target.value)}
          placeholder="Add a new task..."
          style={styles.input}
        />
        <button type="submit" style={styles.addButton}>
          Add Task
        </button>
      </form>

      {/* Filter Buttons */}
      <div style={styles.filters}>
        <button
          onClick={() => setFilter("all")}
          style={{
            ...styles.filterButton,
            backgroundColor: filter === "all" ? "#4CAF50" : "#e0e0e0",
          }}
        >
          All
        </button>
        <button
          onClick={() => setFilter("important")}
          style={{
            ...styles.filterButton,
            backgroundColor: filter === "important" ? "#4CAF50" : "#e0e0e0",
          }}
        >
          Important
        </button>
        <button
          onClick={() => setFilter("completed")}
          style={{
            ...styles.filterButton,
            backgroundColor: filter === "completed" ? "#4CAF50" : "#e0e0e0",
          }}
        >
          Completed
        </button>
      </div>

      {/* Task List */}
      <div style={styles.taskList}>
        {tasks.map((task) => (
          <div key={task.id.toString()} style={styles.taskItem}>
            <input
              type="checkbox"
              checked={task.completed}
              onChange={() => handleToggleComplete(task.id)}
              style={styles.checkbox}
            />
            <span style={{
              ...styles.taskText,
              textDecoration: task.completed ? 'line-through' : 'none'
            }}>
              {task.description}
            </span>
            <div style={styles.taskButtons}>
              <button
                onClick={() => handleToggleImportant(task.id)}
                style={{
                  ...styles.starButton,
                  backgroundColor: task.important ? '#ffd700' : '#e0e0e0'
                }}
              >
                ★
              </button>
              <button
                onClick={() => handleDeleteTask(task.id)}
                style={styles.deleteButton}
              >
                ×
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

// Styles
const styles = {
  container: {
    maxWidth: '600px',
    margin: '0 auto',
    padding: '20px',
    fontFamily: 'Arial, sans-serif',
  },
  title: {
    textAlign: 'center',
    color: '#333',
  },
  form: {
    display: 'flex',
    gap: '10px',
    marginBottom: '20px',
  },
  input: {
    flex: 1,
    padding: '8px',
    fontSize: '16px',
    borderRadius: '4px',
    border: '1px solid #ddd',
  },
  addButton: {
    padding: '8px 16px',
    backgroundColor: '#4CAF50',
    color: 'white',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
  },
  filters: {
    display: 'flex',
    gap: '10px',
    marginBottom: '20px',
  },
  filterButton: {
    padding: '8px 16px',
    color: 'white',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
    flex: 1,
  },
  taskList: {
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
  },
  taskItem: {
    display: 'flex',
    alignItems: 'center',
    padding: '10px',
    backgroundColor: '#f9f9f9',
    borderRadius: '4px',
    gap: '10px',
  },
  checkbox: {
    cursor: 'pointer',
  },
  taskText: {
    flex: 1,
  },
  taskButtons: {
    display: 'flex',
    gap: '5px',
  },
  starButton: {
    padding: '4px 8px',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
  },
  deleteButton: {
    padding: '4px 8px',
    backgroundColor: '#ff4444',
    color: 'white',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
  },
  loading: {
    textAlign: 'center',
    padding: '20px',
    fontSize: '18px',
    color: '#666',
  },
};

export default App;