import { Routes } from '@angular/router';
import { TasksComponent } from './tasks/tasks';
import { UserCardComponent } from './user_card/user-card.component';

export const routes: Routes = [
  { path: '', component: TasksComponent },
  { path: 'user-card', component: UserCardComponent }
];