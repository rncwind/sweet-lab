import React, { useState } from 'react';
import { Project } from './Project';
import ProjectCard from './ProjectCard';
import ProjectForm from './ProjectForm';

interface ProjectListProps {
    projects: Project[];
    onSave: (project: Project) => void;
}

// If we put them in {} we can pass data from tsx to tsx.
function ProjectList({ projects, onSave }: ProjectListProps) {
    const [projectBeingEdited, setProjectBeingEdited] = useState({});

    const handleEdit = (project: Project) => {
        console.log(`setting project to be edited. Project is ${project.name}`);
        setProjectBeingEdited(project);
    };

    const cancelEditing = () => {
        setProjectBeingEdited({});
    };

    const items = projects.map(project => (
        <div key={project.id} className="cols-sm">
            {project === projectBeingEdited ? (
                <ProjectForm onCancel={cancelEditing} onSave={onSave} />
            ) : (
                <ProjectCard project={project} onEdit={handleEdit} />
            )}
        </div>
    ));
    return (
        <div className="row">{items}</div>
    );
}

export default ProjectList;
