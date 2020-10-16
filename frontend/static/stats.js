var ctx = document.getElementById("myChart").getContext("2d");

const colors = {
  green: {
    fill: '#e0eadf',
    stroke: '#5eb84d',
  },
  lightBlue: {
    stroke: '#6fccdd',
  },
  darkBlue: {
    fill: '#92bed2',
    stroke: '#3282bf',
  },
  purple: {
    fill: '#8fa8c8',
    stroke: '#75539e',
  },
};


fetch("/stats/data")
.then(response => response.json())
.then(data => createChart(data))

function createChart(data) {
  const myChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: data["timestamps"],
      datasets: [{
        label: "blue",
        fill: true,
        backgroundColor: colors.purple.fill,
        pointBackgroundColor: colors.purple.stroke,
        borderColor: colors.purple.stroke,
        pointHighlightStroke: colors.purple.stroke,
        borderCapStyle: 'butt',
        data: data["blue"],
  
      }, {
        label: "green",
        fill: true,
        backgroundColor: colors.darkBlue.fill,
        pointBackgroundColor: colors.darkBlue.stroke,
        borderColor: colors.darkBlue.stroke,
        pointHighlightStroke: colors.darkBlue.stroke,
        borderCapStyle: 'butt',
        data: data["green"],
      }]
    },
    options: {
      responsive: false,
      // Can't just just `stacked: true` like the docs say
      scales: {
        yAxes: [{
          stacked: true,
        }]
      },
      animation: {
        duration: 750,
      },
    }
  });
}