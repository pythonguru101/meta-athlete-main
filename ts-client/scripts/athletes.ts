
export interface Athlete {
  id: string,
  name: string,
  kind: string,
  sports: string,
  birthdate: string,
  birthplace: string,
  schoolgrade: string,
  weight: number,
  height: number,
  photo: string,
}

export const TEST_ATHLETES: Athlete[] = [
  {
    id: "1",
    name: "John Doe",
    kind: "Athlete",
    sports: "Rugby",
    birthdate: "09/09/1999",
    birthplace: "San Francisco",
    schoolgrade: "12th grade",
    weight: 82.5,
    height: 1.80,
    photo: ""
  },
  {
    id: "2",
    name: "Bobby Smith",
    kind: "Referee",
    sports: "Volleyball",
    birthdate: "01/01/2000",
    birthplace: "Califonia",
    schoolgrade: "Freshman",
    weight: 78,
    height: 1.83,
    photo: ""
  },
  {
    id: "3",
    name: "Kyle Abrams",
    kind: "Coach",
    sports: "Basketball",
    birthdate: "02/02/2004",
    birthplace: "New York",
    schoolgrade: "Sophomore",
    weight: 75,
    height: 1.75,
    photo: ""
  },
  {
    id: "4",
    name: "David Simpson",
    kind: "Athlete",
    sports: "Tennis",
    birthdate: "05/05/1989",
    birthplace: "Washington",
    schoolgrade: "Senior",
    weight: 85,
    height: 1.90,
    photo: ""
  },
]
